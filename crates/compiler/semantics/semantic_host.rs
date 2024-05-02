use crate::ns::*;

pub struct SemanticHost {
    pub(crate) arena: ThingyArena,

    project_path: Option<String>,
    env_cache: RefCell<Option<Rc<HashMap<String, String>>>>,

    pub(crate) explicit_namespaces: RefCell<HashMap<String, Namespace>>,
    pub(crate) user_namespaces: RefCell<HashMap<String, Namespace>>,
    pub(crate) qnames: RefCell<HashMap<Namespace, HashMap<String, QName>>>,
    void_type: Type,
}

impl SemanticHost {
    pub fn new(options: SemanticHostOptions) -> Self {
        let arena = ThingyArena::new();
        let explicit_namespaces = RefCell::new(HashMap::new());
        let user_namespaces = RefCell::new(HashMap::new());
        let qnames = RefCell::new(HashMap::new());
        let void_type: Type = VoidType::new(&arena).into();
        Self {
            arena,
            project_path: options.project_path.clone(),
            env_cache: RefCell::new(None),
            explicit_namespaces,
            user_namespaces,
            qnames,
            void_type,
        }
    }

    #[inline(always)]
    pub fn factory(&self) -> ThingyFactory {
        ThingyFactory(self)
    }

    pub fn void_type(&self) -> Type {
        self.void_type.clone()
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