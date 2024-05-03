use crate::ns::*;
use bitflags::bitflags;
use smodel::smodel;

smodel! {
    type Arena = ThingyArena;

    /// Unified semantic data type representing
    /// one of several ActionScript 3 variants,
    /// such as classes, variable slots, and reference values.
    pub struct Thingy {
        pub fn defer(&self) -> Result<(), DeferError> {
            if self.is::<UnresolvedThingy>() {
                Err(DeferError::new())
            } else {
                Ok(())
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

        pub fn ns_set_list(&self) -> SharedArray<Namespace> {
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

        pub fn public_ns(&self) -> Option<Namespace> {
            panic!();
        }

        pub fn set_public_ns(&self, ns: Option<Namespace>) {
            panic!();
        }

        pub fn private_ns(&self) -> Option<Namespace> {
            panic!();
        }

        pub fn set_private_ns(&self, ns: Option<Namespace>) {
            panic!();
        }

        pub fn protected_ns(&self) -> Option<Namespace> {
            panic!();
        }

        pub fn set_protected_ns(&self, ns: Option<Namespace>) {
            panic!();
        }

        pub fn static_protected_ns(&self) -> Option<Namespace> {
            panic!();
        }

        pub fn set_static_protected_ns(&self, ns: Option<Namespace>) {
            panic!();
        }

        pub fn internal_ns(&self) -> Option<Namespace> {
            panic!();
        }

        pub fn set_internal_ns(&self, ns: Option<Namespace>) {
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

        pub fn constructor_method(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_constructor_method(&self, m: Option<Thingy>) {}

        pub fn known_subclasses(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn implements(&self) -> SharedArray<Thingy> {
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

        pub fn subpackages(&self) -> SharedMap<String, Package> {
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

        pub fn type_parameters(&self) -> Option<SharedArray<Thingy>> {
            None
        }

        pub fn set_type_parameters(&self, list: Option<SharedArray<Thingy>>) {
        }

        pub fn enum_members(&self) -> SharedMap<String, NumberVariant> {
            panic!();
        }

        pub fn known_implementors(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn extends_interfaces(&self) -> SharedArray<Thingy> {
            panic!();
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
        let ref m_list: SharedArray<Namespace> = SharedArray::new();

        pub(crate) fn NamespaceSet(list: SharedArray<Namespace>) {
            super();
            self.set_m_list(list);
        }

        pub override fn ns_set_list(&self) -> SharedArray<Namespace> {
            self.m_list()
        }
    }

    /// A package consists of a local name, two namespaces, `public` and `internal`,
    /// and a mapping of subpackages.
    pub struct Package: Thingy {
        let ref m_name: String = "".into();
        let ref m_parent: Option<Thingy> = None;
        let ref m_public_ns: Option<Namespace> = None;
        let ref m_internal_ns: Option<Namespace> = None;
        let ref m_properties: NameMap = NameMap::new();
        let ref m_subpackages: SharedMap<String, Package> = SharedMap::new();
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

        pub override fn public_ns(&self) -> Option<Namespace> {
            self.m_public_ns()
        }

        pub override fn set_public_ns(&self, ns: Option<Namespace>) {
            self.set_m_public_ns(ns);
        }

        pub override fn internal_ns(&self) -> Option<Namespace> {
            self.m_internal_ns()
        }

        pub override fn set_internal_ns(&self, ns: Option<Namespace>) {
            self.set_m_internal_ns(ns);
        }

        pub override fn properties(&self, host: &SemanticHost) -> NameMap {
            self.m_properties()
        }

        pub override fn subpackages(&self) -> SharedMap<String, Package> {
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
        let ref m_type_parameters: Option<SharedArray<Thingy>> = None;
        let ref m_extends_class: Option<Thingy> = None;
        let ref m_implements: SharedArray<Thingy> = SharedArray::new();
        let ref m_known_subclasses: SharedArray<Thingy> = SharedArray::new();
        let ref m_constructor_method: Option<Thingy> = None;
        let ref m_parent: Option<Thingy> = None;
        let ref m_private_ns: Option<Namespace> = None;
        let ref m_protected_ns: Option<Namespace> = None;
        let ref m_static_protected_ns: Option<Namespace> = None;
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
            self.m_flex_events().clone()
        }

        pub override fn private_ns(&self) -> Option<Namespace> {
            self.m_private_ns()
        }

        pub override fn set_private_ns(&self, ns: Option<Namespace>) {
            self.set_m_private_ns(ns);
        }

        pub override fn protected_ns(&self) -> Option<Namespace> {
            self.m_protected_ns()
        }

        pub override fn set_protected_ns(&self, ns: Option<Namespace>) {
            self.set_m_protected_ns(ns);
        }

        pub override fn static_protected_ns(&self) -> Option<Namespace> {
            self.m_static_protected_ns()
        }

        pub override fn set_static_protected_ns(&self, ns: Option<Namespace>) {
            self.set_m_static_protected_ns(ns);
        }

        pub override fn type_parameters(&self) -> Option<SharedArray<Thingy>> {
            self.m_type_parameters()
        }

        pub override fn set_type_parameters(&self, list: Option<SharedArray<Thingy>>) {
            self.set_m_type_parameters(list);
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

        pub override fn implements(&self) -> SharedArray<Thingy> {
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

        pub override fn constructor_method(&self) -> Option<Thingy> {
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
            if let Some(type_parameters) = self.type_parameters() {
                p = ".<".to_owned() + &type_parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
            }
            name_1 + &p
        }
    }
    
    pub struct EnumType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_parent: Option<Thingy> = None;
        let ref m_private_ns: Option<Namespace> = None;
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

        pub override fn private_ns(&self) -> Option<Namespace> {
            self.m_private_ns()
        }

        pub override fn set_private_ns(&self, ns: Option<Namespace>) {
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
        let ref m_type_parameters: Option<SharedArray<Thingy>> = None;
        let ref m_extends_interfaces: SharedArray<Thingy> = SharedArray::new();
        let ref m_known_implementors: SharedArray<Thingy> = SharedArray::new();
        let ref m_parent: Option<Thingy> = None;
        let ref m_properties: NameMap = NameMap::new();
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

        pub override fn type_parameters(&self) -> Option<SharedArray<Thingy>> {
            self.m_type_parameters()
        }

        pub override fn set_type_parameters(&self, list: Option<SharedArray<Thingy>>) {
            self.set_m_type_parameters(list);
        }

        pub override fn known_implementors(&self) -> SharedArray<Thingy> {
            self.m_known_implementors()
        }

        pub override fn extends_interfaces(&self) -> SharedArray<Thingy> {
            self.m_extends_interfaces()
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
            let name_1 = self.fully_qualified_name();
            let mut p = String::new();
            if let Some(type_parameters) = self.type_parameters() {
                p = ".<".to_owned() + &type_parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
            }
            name_1 + &p
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
#[derive(Clone)]
pub struct QName(pub(crate) Rc<QName1>);

impl QName {
    pub fn namespace(&self) -> Namespace {
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
    pub(crate) m_namespace: Namespace,
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