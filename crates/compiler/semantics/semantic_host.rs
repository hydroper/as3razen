use crate::ns::*;

pub struct SemanticHost {
    pub(crate) arena: ThingyArena,

    project_path: Option<String>,
    env_cache: RefCell<Option<Rc<HashMap<String, String>>>>,

    pub(crate) explicit_namespaces: RefCell<HashMap<String, Thingy>>,
    pub(crate) user_namespaces: RefCell<HashMap<String, Thingy>>,
    pub(crate) qnames: RefCell<HashMap<Thingy, HashMap<String, QName>>>,
    invalidation_thingy: Thingy,
    unresolved_thingy: Thingy,
    pub(crate) top_level_package: Thingy,
    any_type: Thingy,
    void_type: Thingy,
    object_type: RefCell<Option<Thingy>>,
    boolean_type: RefCell<Option<Thingy>>,
    number_type: RefCell<Option<Thingy>>,
    int_type: RefCell<Option<Thingy>>,
    uint_type: RefCell<Option<Thingy>>,
    float_type: RefCell<Option<Thingy>>,
    string_type: RefCell<Option<Thingy>>,
    array_type: RefCell<Option<Thingy>>,
    namespace_type: RefCell<Option<Thingy>>,
    function_type: RefCell<Option<Thingy>>,

    non_null_primitive_types: RefCell<Option<Rc<Vec<Thingy>>>>,
    pub(crate) types_after_sub: RefCell<HashMap<Thingy, Vec<Thingy>>>,
    pub(crate) function_types: RefCell<HashMap<usize, Vec<Thingy>>>,
    pub(crate) tuple_types: RefCell<HashMap<usize, Vec<Thingy>>>,
    pub(crate) nullable_types: RefCell<HashMap<Thingy, Thingy>>,
    pub(crate) non_nullable_types: RefCell<HashMap<Thingy, Thingy>>,
    /// Variable slots after indirect type substitution.
    pub(crate) vasub: RefCell<HashMap<Thingy, HashMap<SharedArray<Thingy>, Vec<Thingy>>>>,
}

impl SemanticHost {
    pub fn new(options: SemanticHostOptions) -> Self {
        let arena = ThingyArena::new();
        let explicit_namespaces = RefCell::new(HashMap::new());
        let user_namespaces = RefCell::new(HashMap::new());
        let qnames = RefCell::new(HashMap::new());
        let any_type: Thingy = AnyType::new(&arena).into();
        let void_type: Thingy = VoidType::new(&arena).into();
        let invalidation_thingy: Thingy = InvalidationThingy::new(&arena).into();
        let unresolved_thingy: Thingy = UnresolvedThingy::new(&arena).into();
        let top_level_package = Package::new(&arena, "".into());
        let host = Self {
            arena,
            project_path: options.project_path.clone(),
            env_cache: RefCell::new(None),
            explicit_namespaces,
            user_namespaces,
            qnames,
            top_level_package: top_level_package.clone().into(),
            invalidation_thingy,
            unresolved_thingy,

            any_type,
            void_type,
            object_type: RefCell::new(None),
            boolean_type: RefCell::new(None),
            number_type: RefCell::new(None),
            int_type: RefCell::new(None),
            uint_type: RefCell::new(None),
            float_type: RefCell::new(None),
            string_type: RefCell::new(None),
            array_type: RefCell::new(None),
            namespace_type: RefCell::new(None),
            function_type: RefCell::new(None),

            non_null_primitive_types: RefCell::new(None),
            types_after_sub: RefCell::new(HashMap::new()),
            function_types: RefCell::new(HashMap::new()),
            tuple_types: RefCell::new(HashMap::new()),
            nullable_types: RefCell::new(HashMap::new()),
            non_nullable_types: RefCell::new(HashMap::new()),
            vasub: RefCell::new(HashMap::new()),
        };

        // Initialize top level namespaces
        top_level_package.set_public_ns(Some(host.factory().create_public_ns(Some(top_level_package.clone().into()))));
        top_level_package.set_internal_ns(Some(host.factory().create_internal_ns(Some(top_level_package.clone().into()))));

        host
    }

    #[inline(always)]
    pub fn factory(&self) -> ThingyFactory {
        ThingyFactory(self)
    }

    pub fn top_level_package(&self) -> Thingy {
        self.top_level_package.clone()
    }

    pub fn invalidation_thingy(&self) -> Thingy {
        self.invalidation_thingy.clone()
    }

    pub fn unresolved_thingy(&self) -> Thingy {
        self.unresolved_thingy.clone()
    }

    pub fn any_type(&self) -> Thingy {
        self.any_type.clone()
    }

    pub fn void_type(&self) -> Thingy {
        self.void_type.clone()
    }

    global_lookup!(object_type, "Object");
    global_lookup!(boolean_type, "Boolean");
    global_lookup!(number_type, "Number");
    global_lookup!(int_type, "int");
    global_lookup!(uint_type, "uint");
    global_lookup!(float_type, "float");
    global_lookup!(string_type, "String");
    global_lookup!(array_type, "Array");
    global_lookup!(namespace_type, "Namespace");
    global_lookup!(function_type, "Function");

    /// Returns the set of primitive types that do not contain `null`,
    /// such as `Boolean`, `Number`, `int`, `uint`, and `float`.
    /// `String` is never included in the resulting set, as it
    /// includes the `null` value.
    pub fn non_null_primitive_types(&self) -> Result<Rc<Vec<Thingy>>, DeferError> {
        if let Some(r) = self.non_null_primitive_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.boolean_type().defer()?,
            self.number_type().defer()?,
            self.int_type().defer()?,
            self.uint_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.non_null_primitive_types.replace(Some(r.clone()));
        Ok(r)
    }

    /// Preload environment variables from the main project's `.env` file
    /// using the DotEnv file format.
    pub fn env(&self) -> Rc<HashMap<String, String>> {
        if let Some(env) = self.env_cache.borrow().as_ref() {
            return env.clone();
        }
        let mut r = HashMap::<String, String>::new();
        if let Some(project_path) = self.project_path.as_ref() {
            if let Ok(iterator) = dotenvy::from_path_iter(project_path) {
                for item in iterator {
                    if let Ok((key, value)) = item {
                        r.insert(key, value);
                    }
                }
            }
        }
        let r = Rc::new(r);
        self.env_cache.replace(Some(r.clone()));
        r
    }
}

#[derive(Clone)]
pub struct SemanticHostOptions {
    /// The directory path of the main project being compiled,
    /// used for the `import.meta.env.EXAMPLE` accessors.
    pub project_path: Option<String>,
}

impl Default for SemanticHostOptions {
    fn default() -> Self {
        Self {
            project_path: None,
        }
    }
}

macro global_lookup {
    ($field:ident, $as3name:expr) => {
        /// Possibly unresolved.
        pub fn $field(&self) -> Thingy {
            if let Some(r) = self.$field.borrow().as_ref() {
                return r.clone();
            }
            if let Some(r) = self.top_level_package.properties(self).get(&self.factory().create_qname(&self.top_level_package.public_ns().unwrap().into(), $as3name.to_owned())) {
                self.$field.replace(Some(r.clone()));
                r
            } else {
                self.unresolved_thingy()
            }
        }
    },
}