use crate::ns::*;
use bitflags::bitflags;
use smodel::smodel;

smodel! {
    type Arena = ThingyArena;

    /// Unified semantic data type representing
    /// one of several ActionScript 3 variants,
    /// such as classes, variable slots, and reference values.
    pub struct Thingy {
        pub fn defer(&self) -> Result<Thingy, DeferError> {
            if self.is::<UnresolvedThingy>() {
                Err(DeferError::new())
            } else {
                Ok(self.clone())
            }
        }

        pub fn location(&self) -> Option<Location> {
            panic!();
        }

        pub fn set_location(&self, loc: Option<Location>) {
            panic!();
        }

        pub fn system_ns_kind(&self) -> Option<SystemNamespaceKind> {
            None
        }

        pub fn asdoc(&self) -> Option<Rc<AsDoc>> {
            None
        }

        pub fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {}

        pub fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            panic!();
        }

        pub fn uri(&self) -> String {
            "".into()
        }

        pub fn ns_set_list(&self) -> SharedArray<Thingy> {
            panic!()
        }

        pub fn local_name(&self) -> String {
            "".into()
        }

        pub fn parent(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_parent(&self, p: Option<Thingy>) {
            panic!();
        }

        pub fn public_ns(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_public_ns(&self, ns: Option<Thingy>) {
            panic!();
        }

        pub fn private_ns(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_private_ns(&self, ns: Option<Thingy>) {
            panic!();
        }

        pub fn protected_ns(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_protected_ns(&self, ns: Option<Thingy>) {
            panic!();
        }

        pub fn static_protected_ns(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_static_protected_ns(&self, ns: Option<Thingy>) {
            panic!();
        }

        pub fn internal_ns(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_internal_ns(&self, ns: Option<Thingy>) {
            panic!();
        }

        /// Event mapping from `[Event(name="eventName", type="T")]` meta-data.
        pub fn flex_events(&self) -> SharedMap<String, Thingy> {
            panic!();
        }

        pub fn is_abstract(&self) -> bool {
            false
        }

        pub fn set_is_abstract(&self, value: bool) {
        }

        pub fn is_final(&self) -> bool {
            false
        }

        pub fn set_is_final(&self, value: bool) {
        }

        pub fn is_dynamic(&self) -> bool {
            false
        }

        pub fn set_is_dynamic(&self, value: bool) {
        }

        pub fn is_option_set(&self) -> bool {
            false
        }

        pub fn set_is_option_set(&self, value: bool) {
        }

        pub fn constructor_method(&self, host: &SemanticHost) -> Option<Thingy> {
            panic!();
        }

        pub fn set_constructor_method(&self, m: Option<Thingy>) {}

        pub fn known_subclasses(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn implements(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn extends_class(&self, host: &SemanticHost) -> Option<Thingy> {
            panic!();
        }

        pub fn set_extends_class(&self, entity: Option<Thingy>) {
            panic!();
        }

        pub fn prototype(&self, host: &SemanticHost) -> NameMap {
            panic!();
        }

        pub fn properties(&self, host: &SemanticHost) -> NameMap {
            panic!();
        }

        pub fn subpackages(&self) -> SharedMap<String, Thingy> {
            panic!();
        }

        pub fn alias_of(&self) -> Thingy {
            panic!();
        }

        pub fn resolve_alias(&self) -> Thingy {
            self.clone()
        }

        pub fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            panic!();
        }

        pub fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            panic!();
        }

        pub fn name(&self) -> QName {
            panic!();
        }

        pub fn fully_qualified_name(&self) -> String {
            self.fully_qualified_name_list().join(".")
        }
    
        pub fn fully_qualified_name_list(&self) -> Vec<String> {
            let mut r: Vec<String> = vec![];
            let mut p = Some(self.clone());
            while let Some(p1) = p {
                let name = if p1.is::<Package>() {
                    p1.local_name()
                } else {
                    p1.name().to_string()
                };
                if !name.is_empty() {
                    r.insert(0, name);
                }
                p = p1.parent();
            }
            r
        }

        pub fn type_params(&self) -> Option<SharedArray<Thingy>> {
            None
        }

        pub fn set_type_params(&self, list: Option<SharedArray<Thingy>>) {
        }

        pub fn enum_members(&self) -> SharedMap<String, NumberVariant> {
            panic!();
        }

        pub fn known_implementors(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn extends_interfaces(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn origin(&self) -> Thingy {
            panic!();
        }

        pub fn substitute_types(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn element_types(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn params(&self) -> SharedArray<Rc<SemanticFunctionTypeParameter>> {
            panic!();
        }

        pub fn result_type(&self) -> Thingy {
            panic!();
        }

        pub fn base(&self) -> Thingy {
            panic!();
        }

        /// Performs type substitution.
        pub fn type_substitution(&self, host: &SemanticHost, type_params: &SharedArray<Thingy>, substitute_types: &SharedArray<Thingy>) -> Thingy {
            TypeSubstitution(host).exec(self, type_params, substitute_types)
        }

        fn to_string_1(&self) -> String {
            "".into()
        }
    }

    pub struct UnresolvedThingy: Thingy {
        pub(crate) fn UnresolvedThingy() {
            super();
        }
    }

    /// Thingy used to indicate that an entity is invalidated.
    pub struct InvalidationThingy: Thingy {
        pub(crate) fn InvalidationThingy() {
            super();
        }
    }

    pub struct Namespace: Thingy {
        pub(crate) fn Namespace() {
            super();
        }
    }

    pub struct SystemNamespace: Namespace {
        let m_kind: SystemNamespaceKind = SystemNamespaceKind::Public;
        let ref m_parent: Option<Thingy> = None;

        pub(crate) fn SystemNamespace(kind: SystemNamespaceKind, parent: Option<Thingy>) {
            super();
            self.set_m_kind(kind);
            self.set_m_parent(parent);
        }

        pub override fn system_ns_kind(&self) -> Option<SystemNamespaceKind> {
            Some(self.m_kind())
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        override fn to_string_1(&self) -> String {
            self.m_kind().to_string()
        }
    }

    pub struct UserNamespace: Namespace {
        let ref m_uri: String = "".into();

        pub(crate) fn UserNamespace(uri: String) {
            super();
            self.set_m_uri(uri);
        }

        pub override fn uri(&self) -> String {
            self.m_uri()
        }

        override fn to_string_1(&self) -> String {
            self.m_uri()
        }
    }

    pub struct ExplicitNamespace: Namespace {
        let ref m_uri: String = "".into();

        pub(crate) fn ExplicitNamespace(uri: String) {
            super();
            self.set_m_uri(uri);
        }

        pub override fn uri(&self) -> String {
            self.m_uri()
        }

        override fn to_string_1(&self) -> String {
            self.m_uri()
        }
    }

    pub struct NamespaceSet: Thingy {
        let ref m_list: SharedArray<Thingy> = SharedArray::new();

        pub(crate) fn NamespaceSet(list: SharedArray<Thingy>) {
            super();
            self.set_m_list(list);
        }

        pub override fn ns_set_list(&self) -> SharedArray<Thingy> {
            self.m_list()
        }
    }

    /// A package consists of a local name, two namespaces, `public` and `internal`,
    /// and a mapping of subpackages.
    pub struct Package: Thingy {
        let ref m_name: String = "".into();
        let ref m_parent: Option<Thingy> = None;
        let ref m_public_ns: Option<Thingy> = None;
        let ref m_internal_ns: Option<Thingy> = None;
        let ref m_properties: NameMap = NameMap::new();
        let ref m_subpackages: SharedMap<String, Thingy> = SharedMap::new();
        let ref m_asdoc: Option<Rc<AsDoc>> = None;

        pub(crate) fn Package(name: String) {
            super();
            self.set_m_name(name);
        }

        /// The local name of the package. For the top-level package
        /// this is the empty string.
        pub override fn local_name(&self) -> String {
            self.m_name()
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Thingy>) {
            self.set_m_parent(p);
        }

        pub override fn public_ns(&self) -> Option<Thingy> {
            self.m_public_ns()
        }

        pub override fn set_public_ns(&self, ns: Option<Thingy>) {
            self.set_m_public_ns(ns);
        }

        pub override fn internal_ns(&self) -> Option<Thingy> {
            self.m_internal_ns()
        }

        pub override fn set_internal_ns(&self, ns: Option<Thingy>) {
            self.set_m_internal_ns(ns);
        }

        pub override fn properties(&self, host: &SemanticHost) -> NameMap {
            self.m_properties()
        }

        pub override fn subpackages(&self) -> SharedMap<String, Thingy> {
            self.m_subpackages()
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct Alias: Thingy {
        let ref m_name: Option<QName> = None;
        let ref m_alias_of: Option<Thingy> = None;
        let ref m_parent: Option<Thingy> = None;
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;

        pub(crate) fn Alias(name: QName, alias_of: Thingy) {
            super();
            self.set_m_name(Some(name));
            self.set_m_alias_of(Some(alias_of));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn alias_of(&self) -> Thingy {
            self.m_alias_of().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn resolve_alias(&self) -> Thingy {
            self.alias_of().resolve_alias()
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Thingy>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        override fn to_string_1(&self) -> String {
            self.alias_of().to_string_1()
        }
    }

    pub struct Type: Thingy {
        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }
    }

    pub struct AnyType : Type {
        pub(crate) fn AnyType() {
            super();
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            "*".into()
        }
    }

    pub struct VoidType : Type {
        pub(crate) fn VoidType() {
            super();
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        override fn to_string_1(&self) -> String {
            "void".into()
        }
    }

    pub struct ClassType: Type {
        let ref m_name: Option<QName> = None;
        let m_flags: ClassTypeFlags = ClassTypeFlags::empty();
        let ref m_type_params: Option<SharedArray<Thingy>> = None;
        let ref m_extends_class: Option<Thingy> = None;
        let ref m_implements: SharedArray<Thingy> = SharedArray::new();
        let ref m_known_subclasses: SharedArray<Thingy> = SharedArray::new();
        let ref m_constructor_method: Option<Thingy> = None;
        let ref m_parent: Option<Thingy> = None;
        let ref m_private_ns: Option<Thingy> = None;
        let ref m_protected_ns: Option<Thingy> = None;
        let ref m_static_protected_ns: Option<Thingy> = None;
        let ref m_properties: NameMap = NameMap::new();
        let ref m_prototype: NameMap = NameMap::new();
        let ref m_flex_events: SharedMap<String, Thingy> = SharedMap::new();
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;

        pub(crate) fn ClassType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        #[inheritdoc]
        pub override fn flex_events(&self) -> SharedMap<String, Thingy> {
            self.m_flex_events()
        }

        pub override fn private_ns(&self) -> Option<Thingy> {
            self.m_private_ns()
        }

        pub override fn set_private_ns(&self, ns: Option<Thingy>) {
            self.set_m_private_ns(ns);
        }

        pub override fn protected_ns(&self) -> Option<Thingy> {
            self.m_protected_ns()
        }

        pub override fn set_protected_ns(&self, ns: Option<Thingy>) {
            self.set_m_protected_ns(ns);
        }

        pub override fn static_protected_ns(&self) -> Option<Thingy> {
            self.m_static_protected_ns()
        }

        pub override fn set_static_protected_ns(&self, ns: Option<Thingy>) {
            self.set_m_static_protected_ns(ns);
        }

        pub override fn type_params(&self) -> Option<SharedArray<Thingy>> {
            self.m_type_params()
        }

        pub override fn set_type_params(&self, list: Option<SharedArray<Thingy>>) {
            self.set_m_type_params(list);
        }

        pub override fn is_abstract(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_ABSTRACT)
        }

        pub override fn set_is_abstract(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_ABSTRACT, value);
            self.set_m_flags(v);
        }

        pub override fn is_final(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_FINAL)
        }

        pub override fn set_is_final(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_FINAL, value);
            self.set_m_flags(v);
        }

        pub override fn is_dynamic(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_DYNAMIC)
        }

        pub override fn set_is_dynamic(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_DYNAMIC, value);
            self.set_m_flags(v);
        }

        /// Whether the class is an `[OptionSet]` class.
        pub override fn is_option_set(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_OPTION_SET)
        }

        pub override fn set_is_option_set(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_OPTION_SET, value);
            self.set_m_flags(v);
        }

        pub override fn known_subclasses(&self) -> SharedArray<Thingy> {
            self.m_known_subclasses()
        }

        pub override fn implements(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            self.m_implements()
        }

        pub override fn extends_class(&self, host: &SemanticHost) -> Option<Thingy> {
            self.m_extends_class()
        }

        pub override fn set_extends_class(&self, entity: Option<Thingy>) {
            self.set_m_extends_class(entity);
        }

        pub override fn properties(&self, host: &SemanticHost) -> NameMap {
            self.m_properties()
        }

        pub override fn prototype(&self, host: &SemanticHost) -> NameMap {
            self.m_prototype()
        }

        pub override fn constructor_method(&self, host: &SemanticHost) -> Option<Thingy> {
            self.m_constructor_method()
        }

        pub override fn set_constructor_method(&self, m: Option<Thingy>) {
            self.set_m_constructor_method(m);
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Thingy>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(!host.non_null_primitive_types()?.contains(&self.clone().into()))
        }

        override fn to_string_1(&self) -> String {
            let name_1 = self.fully_qualified_name();
            let mut p = String::new();
            if let Some(type_params) = self.type_params() {
                p = ".<".to_owned() + &type_params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
            }
            name_1 + &p
        }
    }
    
    pub struct EnumType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_parent: Option<Thingy> = None;
        let ref m_private_ns: Option<Thingy> = None;
        let ref m_properties: NameMap = NameMap::new();
        let ref m_prototype: NameMap = NameMap::new();
        let ref m_members: SharedMap<String, NumberVariant> = SharedMap::new();
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;

        pub(crate) fn EnumType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn enum_members(&self) -> SharedMap<String, NumberVariant> {
            self.m_members()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn private_ns(&self) -> Option<Thingy> {
            self.m_private_ns()
        }

        pub override fn set_private_ns(&self, ns: Option<Thingy>) {
            self.set_m_private_ns(ns);
        }

        pub override fn is_abstract(&self) -> bool {
            false
        }

        pub override fn is_final(&self) -> bool {
            true
        }

        pub override fn is_dynamic(&self) -> bool {
            false
        }

        pub override fn is_option_set(&self) -> bool {
            false
        }

        pub override fn extends_class(&self, host: &SemanticHost) -> Option<Thingy> {
            Some(host.object_type())
        }

        pub override fn properties(&self, host: &SemanticHost) -> NameMap {
            self.m_properties()
        }

        pub override fn prototype(&self, host: &SemanticHost) -> NameMap {
            self.m_prototype()
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Thingy>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct InterfaceType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_type_params: Option<SharedArray<Thingy>> = None;
        let ref m_extends_interfaces: SharedArray<Thingy> = SharedArray::new();
        let ref m_known_implementors: SharedArray<Thingy> = SharedArray::new();
        let ref m_parent: Option<Thingy> = None;
        let ref m_prototype: NameMap = NameMap::new();
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;

        pub(crate) fn InterfaceType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn type_params(&self) -> Option<SharedArray<Thingy>> {
            self.m_type_params()
        }

        pub override fn set_type_params(&self, list: Option<SharedArray<Thingy>>) {
            self.set_m_type_params(list);
        }

        pub override fn known_implementors(&self) -> SharedArray<Thingy> {
            self.m_known_implementors()
        }

        pub override fn extends_interfaces(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            self.m_extends_interfaces()
        }

        pub override fn prototype(&self, host: &SemanticHost) -> NameMap {
            self.m_prototype()
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Thingy>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            let name_1 = self.fully_qualified_name();
            let mut p = String::new();
            if let Some(type_params) = self.type_params() {
                p = ".<".to_owned() + &type_params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
            }
            name_1 + &p
        }
    }

    /// Type after substitution, whose origin is either
    /// a class or an interface. Other types, after substitution,
    /// such as structural types, are represented by their
    /// same type with substitution in compound parts.
    pub struct TypeAfterSubstitution: Type {
        let ref m_origin: Option<Thingy> = None;
        let ref m_substitute_types: SharedArray<Thingy> = SharedArray::new();
        let ref m_extends_class: Option<Thingy> = None;
        let ref m_implements: Option<SharedArray<Thingy>> = None;
        let ref m_extends_interfaces: Option<SharedArray<Thingy>> = None;
        let ref m_properties: Option<NameMap> = None;
        let ref m_prototype: Option<NameMap> = None;
        let ref m_constructor_method: Option<Thingy> = None;

        pub(crate) fn TypeAfterSubstitution(origin: Thingy, substitute_types: SharedArray<Thingy>) {
            super();
            self.set_m_origin(Some(origin));
            self.set_m_substitute_types(substitute_types);
        }

        pub override fn origin(&self) -> Thingy {
            self.m_origin().unwrap()
        }

        pub override fn substitute_types(&self) -> SharedArray<Thingy> {
            self.m_substitute_types()
        }
        
        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        pub override fn flex_events(&self) -> SharedMap<String, Thingy> {
            self.origin().flex_events()
        }

        pub override fn is_abstract(&self) -> bool {
            self.origin().is_abstract()
        }

        pub override fn is_final(&self) -> bool {
            self.origin().is_final()
        }

        pub override fn is_dynamic(&self) -> bool {
            self.origin().is_dynamic()
        }

        pub override fn is_option_set(&self) -> bool {
            self.origin().is_option_set()
        }

        pub override fn extends_class(&self, host: &SemanticHost) -> Option<Thingy> {
            if let Some(r) = self.m_extends_class() {
                return Some(r.clone());
            }
            let origin = self.origin();
            let r = origin.extends_class(host);
            if r.is_none() {
                return None;
            }
            let r = r.unwrap();
            if r.is::<UnresolvedThingy>() {
                return Some(r.clone());
            }
            let r = TypeSubstitution(host).exec(&r, &origin.type_params().unwrap(), &self.m_substitute_types());
            self.set_m_extends_class(Some(r.clone()));
            Some(r)
        }

        pub override fn implements(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            if let Some(r) = self.m_implements() {
                return r;
            }
            let origin = self.origin();
            let r: SharedArray<Thingy> = origin.implements(host).iter().map(|t| TypeSubstitution(host).exec(&t, &origin.type_params().unwrap(), &self.m_substitute_types())).collect();
            self.set_m_implements(Some(r.clone()));
            r
        }

        pub override fn extends_interfaces(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            if let Some(r) = self.m_extends_interfaces() {
                return r;
            }
            let origin = self.origin();
            let r: SharedArray<Thingy> = origin.extends_interfaces(host).iter().map(|t| TypeSubstitution(host).exec(&t, &origin.type_params().unwrap(), &self.m_substitute_types())).collect();
            self.set_m_extends_interfaces(Some(r.clone()));
            r
        }

        pub override fn prototype(&self, host: &SemanticHost) -> NameMap {
            if let Some(r) = self.m_prototype() {
                return r;
            }
            let origin = self.origin();
            let mut r = NameMap::new();
            for (name, thingy) in origin.prototype(host).borrow().iter() {
                let thingy = TypeSubstitution(host).exec(&thingy, &origin.type_params().unwrap(), &self.m_substitute_types());
                r.set(name.clone(), thingy)
            }
            self.set_m_prototype(Some(r.clone()));
            r
        }

        pub override fn properties(&self, host: &SemanticHost) -> NameMap {
            if let Some(r) = self.m_properties() {
                return r;
            }
            let origin = self.origin();
            let mut r = NameMap::new();
            for (name, thingy) in origin.properties(host).borrow().iter() {
                let thingy = TypeSubstitution(host).exec(&thingy, &origin.type_params().unwrap(), &self.m_substitute_types());
                r.set(name.clone(), thingy)
            }
            self.set_m_properties(Some(r.clone()));
            r
        }

        pub override fn constructor_method(&self, host: &SemanticHost) -> Option<Thingy> {
            if let Some(r) = self.m_constructor_method() {
                return Some(r.clone());
            }
            let origin = self.origin();
            let r = origin.constructor_method(host);
            if r.is_none() {
                return None;
            }
            let r = r.unwrap();
            let r = TypeSubstitution(host).exec(&r, &origin.type_params().unwrap(), &self.m_substitute_types());
            self.set_m_constructor_method(Some(r.clone()));
            Some(r)
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.origin().parent()
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.origin().asdoc()
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.origin().metadata()
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            let name_1 = self.fully_qualified_name();
            let a = self.m_substitute_types();
            let p = ".<".to_owned() + &a.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ") + ">";
            name_1 + &p
        }
    }

    /// Tuple type. The tuple type is equivalent to
    /// `Array` with type safety for its element types.
    pub struct TupleType: Type {
        let ref m_elements: SharedArray<Thingy> = SharedArray::new();

        pub(crate) fn TupleType(elements: SharedArray<Thingy>) {
            super();
            self.set_m_elements(elements);
        }
        
        pub override fn element_types(&self) -> SharedArray<Thingy> {
            self.m_elements()
        }

        pub override fn is_abstract(&self) -> bool {
            false
        }

        pub override fn is_final(&self) -> bool {
            true
        }

        pub override fn is_dynamic(&self) -> bool {
            true
        }

        pub override fn is_option_set(&self) -> bool {
            false
        }

        pub override fn extends_class(&self, host: &SemanticHost) -> Option<Thingy> {
            Some(host.array_type())
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            format!("[{}]", self.element_types().iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
        }
    }

    /// Structural function type. This type is equivalent to `Function`
    /// with type safety.
    pub struct FunctionType: Type {
        let ref m_params: SharedArray<Rc<SemanticFunctionTypeParameter>> = SharedArray::new();
        let ref m_result_type: Option<Thingy> = None;

        pub(crate) fn FunctionType(params: SharedArray<Rc<SemanticFunctionTypeParameter>>, result_type: Thingy) {
            super();
            self.set_m_params(params);
            self.set_m_result_type(Some(result_type));
        }
        
        pub override fn params(&self) -> SharedArray<Rc<SemanticFunctionTypeParameter>> {
            self.m_params()
        }

        pub override fn result_type(&self) -> Thingy {
            self.m_result_type().unwrap()
        }

        pub override fn is_abstract(&self) -> bool {
            false
        }

        pub override fn is_final(&self) -> bool {
            true
        }

        pub override fn is_dynamic(&self) -> bool {
            false
        }

        pub override fn is_option_set(&self) -> bool {
            false
        }

        pub override fn extends_class(&self, host: &SemanticHost) -> Option<Thingy> {
            Some(host.function_type())
        }

        pub override fn includes_undefined(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &SemanticHost) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            let mut p = Vec::<String>::new();
            for p1 in self.params().iter() {
                match p1.kind {
                    ParameterKind::Required => {
                        p.push(p1.static_type.to_string());
                    },
                    ParameterKind::Optional => {
                        p.push(p1.static_type.to_string() + &"=".to_owned());
                    },
                    ParameterKind::Rest => {
                        p.push("...".to_owned() + &p1.static_type.to_string());
                    },
                }
            }
            format!("function({}) : {}", p.join(", "), self.result_type().to_string())
        }
    }

    /// The nullable type `T?`. It is equivalent to either
    /// `T` or `*` (for all primitive types but String).
    pub struct NullableType: Type {
        let ref m_base: Option<Thingy> = None;

        pub(crate) fn NullableType(base: Thingy) {
            super();
            self.set_m_base(Some(base));
        }

        /// The type that is made nullable.
        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn includes_undefined(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            if let Ok(ft) = self.base().to::<FunctionType>() {
                format!("?{}", ft.to_string())
            } else {
                format!("{}?", self.base().to_string())
            }
        }
    }

    pub struct NonNullableType: Type {
        let ref m_base: Option<Thingy> = None;

        pub(crate) fn NonNullableType(base: Thingy) {
            super();
            self.set_m_base(Some(base));
        }

        /// The type that is made non-nullable.
        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn includes_undefined(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        override fn to_string_1(&self) -> String {
            if let Ok(ft) = self.base().to::<FunctionType>() {
                format!("({})!", ft.to_string())
            } else {
                format!("{}!", self.base().to_string())
            }
        }
    }

    pub struct TypeParameterType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn TypeParameterType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn includes_undefined(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        override fn to_string_1(&self) -> String {
            self.name().to_string()
        }
    }
}

impl ToString for Thingy {
    fn to_string(&self) -> String {
        self.to_string_1()
    }
}

impl DiagnosticArgument for Thingy {}

#[derive(Copy, Clone, PartialEq)]
pub enum SystemNamespaceKind {
    Public,
    Private,
    Protected,
    Internal,
    StaticProtected,
}

impl ToString for SystemNamespaceKind {
    fn to_string(&self) -> String {
        match self {
            Self::Public => "public".into(),
            Self::Private => "private".into(),
            Self::Protected => "protected".into(),
            Self::Internal => "internal".into(),
            Self::StaticProtected => "static protected".into(),
        }
    }
}

/// A qualified name in ActionScript 3 consisting of
/// a namespace and a local name.
/// 
/// This structure is not intended for E4X, but for representing
/// ActionScript 3 property names.
/// 
/// # Representation
/// 
/// `QName` in this codebase is a type managed by reference counting.
/// Calling `.clone()` in it will clone by reference, not by content.
#[derive(Clone)]
pub struct QName(pub(crate) Rc<QName1>);

impl QName {
    pub fn namespace(&self) -> Thingy {
        self.0.m_namespace.clone()
    }

    pub fn local_name(&self) -> String {
        self.0.m_local_name.clone()
    }
}

impl std::hash::Hash for QName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for QName {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for QName {}

pub(crate) struct QName1 {
    pub(crate) m_namespace: Thingy,
    pub(crate) m_local_name: String,
}

impl ToString for QName {
    fn to_string(&self) -> String {
        let q = self.namespace();
        let lname = self.local_name();
        if q.is::<SystemNamespace>() {
            return lname;
        }
        format!("{}::{lname}", q.uri())
    }
}

impl DiagnosticArgument for QName {}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct ClassTypeFlags: u16 {
        const IS_FINAL = 0b00000001;
        const IS_STATIC = 0b00000010;
        const IS_ABSTRACT = 0b00000100;
        const IS_DYNAMIC = 0b00001000;
        const IS_OPTION_SET = 0b00010000;
    }
}

/// Parameter belonging to a function type in the semantic model.
pub struct SemanticFunctionTypeParameter {
    pub kind: ParameterKind,
    /// Static type of the parameter. It is never `UnresolvedThingy`
    /// as function types are only created after all compound types
    /// are resolved.
    pub static_type: Thingy,
}

impl SemanticFunctionTypeParameter {
    /// Performs type substitution.
    pub fn type_substitution(&self, host: &SemanticHost, type_params: &SharedArray<Thingy>, substitute_types: &SharedArray<Thingy>) -> Self {
        Self {
            kind: self.kind,
            static_type: TypeSubstitution(host).exec(&self.static_type, type_params, substitute_types),
        }
    }
}