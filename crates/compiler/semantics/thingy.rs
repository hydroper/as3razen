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

        pub fn qualifier(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn key(&self) -> Thingy {
            panic!();
        }

        /// Returns the static type of a property, whether for a type, variable, virtual or method slot or namespace.
        pub fn property_static_type(&self, host: &SemanticHost) -> Thingy {
            panic!();
        }

        pub fn tuple_index(&self) -> usize {
            0
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

        pub fn object(&self) -> Thingy {
            panic!();
        }

        pub fn uri(&self) -> String {
            "".into()
        }

        pub fn open_ns_set(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn import_list(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn local_name(&self) -> String {
            "".into()
        }

        pub fn class(&self) -> Thingy {
            panic!();
        }

        pub fn interface(&self) -> Thingy {
            panic!();
        }

        pub fn package(&self) -> Thingy {
            panic!();
        }

        pub fn of_method(&self) -> Thingy {
            panic!();
        }
        
        pub fn this(&self) -> Option<Thingy> {
            panic!();
        }
        
        pub fn set_this(&self, this: Option<Thingy>) {
            panic!();
        }
        
        pub fn property_has_capture(&self, property: &Thingy) -> bool {
            panic!();
        }
        
        pub fn set_property_has_capture(&self, property: &Thingy, value: bool) {
            panic!();
        }

        pub fn referenced_type(&self) -> Thingy {
            panic!();
        }

        pub fn parent(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_parent(&self, p: Option<Thingy>) {
            panic!();
        }

        pub fn package_concats(&self) -> SharedArray<Thingy> {
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

        pub fn is_public_ns(&self) -> bool {
            false
        }

        pub fn is_private_ns(&self) -> bool {
            false
        }

        pub fn is_protected_ns(&self) -> bool {
            false
        }

        pub fn is_internal_ns(&self) -> bool {
            false
        }

        pub fn is_static_protected_ns(&self) -> bool {
            false
        }

        pub fn number_value(&self) -> NumberVariant {
            panic!();
        }

        pub fn string_value(&self) -> String {
            panic!();
        }

        pub fn boolean_value(&self) -> bool {
            panic!();
        }

        /// Returns whether a type is a class, whether
        /// original or after substitution.
        pub fn is_class_type_possibly_after_sub(&self) -> bool {
            false
        }

        /// Returns whether a type is an interface, whether
        /// original or after substitution.
        pub fn is_interface_type_possibly_after_sub(&self) -> bool {
            false
        }

        /// Event mapping from `[Event(name="eventName", type="T")]` meta-data.
        pub fn flex_events(&self) -> SharedMap<String, Thingy> {
            panic!();
        }

        pub fn bindable_event(&self) -> Option<String> {
            panic!();
        }

        pub fn set_bindable_event(&self, name: Option<String>) {
            panic!();
        }

        pub fn getter(&self, host: &SemanticHost) -> Option<Thingy> {
            panic!();
        }

        pub fn set_getter(&self, m: Option<Thingy>) {
            panic!();
        }

        pub fn setter(&self, host: &SemanticHost) -> Option<Thingy> {
            panic!();
        }

        pub fn set_setter(&self, m: Option<Thingy>) {
            panic!();
        }

        pub fn static_type(&self, host: &SemanticHost) -> Thingy {
            panic!();
        }

        pub fn set_static_type(&self, value: Thingy) {
            panic!();
        }

        pub fn clone_constant(&self, host: &SemanticHost) -> Thingy {
            panic!();
        }

        pub fn is_external(&self) -> bool {
            false
        }

        pub fn set_is_external(&self, value: bool) {
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

        pub fn is_static(&self) -> bool {
            false
        }

        pub fn set_is_static(&self, value: bool) {
        }

        pub fn is_overriding(&self) -> bool {
            false
        }

        pub fn set_is_overriding(&self, value: bool) {
        }

        pub fn is_async(&self) -> bool {
            false
        }

        pub fn set_is_async(&self, value: bool) {
        }

        pub fn is_generator(&self) -> bool {
            false
        }

        pub fn set_is_generator(&self, value: bool) {
        }

        pub fn signature(&self, host: &SemanticHost) -> Thingy {
            panic!();
        }

        pub fn set_signature(&self, signature: &Thingy) {
            panic!();
        }

        pub fn activation(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_activation(&self, activation: Option<Thingy>) {
            panic!();
        }

        pub fn of_virtual_slot(&self, host: &SemanticHost) -> Option<Thingy> {
            panic!();
        }

        pub fn set_of_virtual_slot(&self, virtual_slot: Option<Thingy>) {
            panic!();
        }

        pub fn overriden_by(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn overrides_method(&self, host: &SemanticHost) -> Option<Thingy> {
            panic!();
        }

        pub fn set_overrides_method(&self, method: Option<Thingy>) {
            panic!();
        }

        pub fn is_constructor(&self) -> bool {
            false
        }

        pub fn set_is_constructor(&self, value: bool) {
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

        pub fn property(&self) -> Thingy {
            panic!();
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

        pub fn indirect_type_params(&self) -> SharedArray<Thingy> {
            panic!();
        }

        pub fn indirect_substitute_types(&self) -> SharedArray<Thingy> {
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

        pub fn read_only(&self, host: &SemanticHost) -> bool {
            true
        }

        pub fn set_read_only(&self, value: bool) {
            panic!();
        }

        pub fn write_only(&self, host: &SemanticHost) -> bool {
            false
        }

        pub fn set_write_only(&self, value: bool) {
            panic!();
        }

        pub fn var_constant(&self) -> Option<Thingy> {
            panic!();
        }

        pub fn set_var_constant(&self, k: Option<Thingy>) {
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

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &SemanticHost) -> Thingy {
            host.namespace_type()
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

        pub override fn is_public_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Public
        }

        pub override fn is_private_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Private
        }

        pub override fn is_protected_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Protected
        }

        pub override fn is_internal_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Internal
        }

        pub override fn is_static_protected_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::StaticProtected
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

    /// A package consists of a local name, two namespaces, `public` and `internal`,
    /// and a mapping of subpackages.
    pub struct Package: Thingy {
        let ref m_name: String = "".into();
        let ref m_parent: Option<Thingy> = None;
        let ref m_public_ns: Option<Thingy> = None;
        let ref m_internal_ns: Option<Thingy> = None;
        let ref m_properties: NameMap = NameMap::new();
        let ref m_subpackages: SharedMap<String, Thingy> = SharedMap::new();
        let ref m_package_concats: SharedArray<Thingy> = SharedArray::new();
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

        /// Concatenated packages.
        pub override fn package_concats(&self) -> SharedArray<Thingy> {
            self.m_package_concats()
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
        let m_is_external: bool = false;
        let ref m_parent: Option<Thingy> = None;
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

        pub override fn is_external(&self) -> bool {
            self.m_is_external()
        }

        pub override fn set_is_external(&self, value: bool) {
            self.set_m_is_external(value);
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

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &SemanticHost) -> Thingy {
            host.class_type()
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

        pub override fn is_class_type_possibly_after_sub(&self) -> bool {
            true
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

        pub override fn is_static(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_STATIC)
        }

        pub override fn set_is_static(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_STATIC, value);
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

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_EXTERNAL, value);
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
        let m_is_external: bool = false;
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

        pub override fn is_external(&self) -> bool {
            self.m_is_external()
        }

        pub override fn set_is_external(&self, value: bool) {
            self.set_m_is_external(value);
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
        let m_is_external: bool = false;
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

        pub override fn is_interface_type_possibly_after_sub(&self) -> bool {
            true
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

        pub override fn is_external(&self) -> bool {
            self.m_is_external()
        }

        pub override fn set_is_external(&self, value: bool) {
            self.set_m_is_external(value);
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

        pub override fn is_class_type_possibly_after_sub(&self) -> bool {
            self.origin().is_class_type_possibly_after_sub()
        }

        pub override fn is_interface_type_possibly_after_sub(&self) -> bool {
            self.origin().is_interface_type_possibly_after_sub()
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

        pub override fn is_static(&self) -> bool {
            self.origin().is_static()
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

        pub override fn location(&self) -> Option<Location> {
            None
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

    /// Either an *original* variable slot, or a variable slot after substitution.
    pub struct VariableSlot: Thingy {
        fn VariableSlot() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &SemanticHost) -> Thingy {
            self.static_type(host)
        }
    }

    pub struct OriginalVariableSlot: VariableSlot {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_constant: Option<Thingy> = None;
        let ref m_static_type: Option<Thingy> = None;
        let ref m_parent: Option<Thingy> = None;
        let m_flags: VariableSlotFlags = VariableSlotFlags::empty();
        let ref m_bindable_event: Option<String> = None;

        pub(crate) fn OriginalVariableSlot(name: &QName, read_only: bool, static_type: &Thingy) {
            super();
            self.set_m_name(Some(name.clone()));
            self.set_read_only(read_only);
            self.set_m_static_type(Some(static_type.clone()));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        /// The constant initially assigned to that variable slot.
        pub override fn var_constant(&self) -> Option<Thingy> {
            self.m_constant()
        }

        /// The constant initially assigned to that variable slot.
        pub override fn set_var_constant(&self, k: Option<Thingy>) {
            self.set_m_constant(k);
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(VariableSlotFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(VariableSlotFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        pub override fn read_only(&self, host: &SemanticHost) -> bool {
            self.m_flags().contains(VariableSlotFlags::READ_ONLY)
        }

        pub override fn set_read_only(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(VariableSlotFlags::READ_ONLY, value);
            self.set_m_flags(v);
        }

        pub override fn write_only(&self, host: &SemanticHost) -> bool {
            false
        }

        pub override fn static_type(&self, host: &SemanticHost) -> Thingy {
            self.m_static_type().unwrap()
        }

        pub override fn set_static_type(&self, value: Thingy) {
            self.set_m_static_type(Some(value));
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        /// The event name indicated by a `[Bindable]` meta-data tag.
        pub override fn bindable_event(&self) -> Option<String> {
            self.m_bindable_event()
        }

        pub override fn set_bindable_event(&self, name: Option<String>) {
            self.set_m_bindable_event(name);
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
            self.fully_qualified_name()
        }
    }

    /// Variable slot after indirect substitution.
    pub struct VariableSlotAfterSubstitution: VariableSlot {
        let ref m_origin: Option<Thingy> = None;
        let ref m_indirect_type_params: SharedArray<Thingy> = SharedArray::new();
        let ref m_indirect_substitute_types: SharedArray<Thingy> = SharedArray::new();
        let ref m_static_type: Option<Thingy> = None;

        pub(crate) fn VariableSlotAfterSubstitution(origin: &Thingy, indirect_type_params: &SharedArray<Thingy>, indirect_substitute_types: &SharedArray<Thingy>) {
            super();
            self.set_m_origin(Some(origin.clone()));
            self.set_m_indirect_type_params(indirect_type_params.clone());
            self.set_m_indirect_substitute_types(indirect_substitute_types.clone());
        }

        pub override fn origin(&self) -> Thingy {
            self.m_origin().unwrap()
        }

        pub override fn indirect_type_params(&self) -> SharedArray<Thingy> {
            self.m_indirect_type_params()
        }

        pub override fn indirect_substitute_types(&self) -> SharedArray<Thingy> {
            self.m_indirect_substitute_types()
        }

        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        /// The constant initially assigned to that variable slot.
        pub override fn var_constant(&self) -> Option<Thingy> {
            None
        }

        pub override fn is_external(&self) -> bool {
            self.origin().is_external()
        }

        pub override fn read_only(&self, host: &SemanticHost) -> bool {
            self.origin().read_only(host)
        }

        pub override fn write_only(&self, host: &SemanticHost) -> bool {
            false
        }

        pub override fn static_type(&self, host: &SemanticHost) -> Thingy {
            if let Some(r) = self.m_static_type() {
                return r.clone();
            }
            let r = self.origin().static_type(host);
            if r.is::<UnresolvedThingy>() {
                return r.clone();
            }
            let r = TypeSubstitution(host).exec(&r, &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_static_type(Some(r.clone()));
            r
        }

        pub override fn location(&self) -> Option<Location> {
            None
        }

        /// The event name indicated by a `[Bindable]` meta-data tag.
        pub override fn bindable_event(&self) -> Option<String> {
            self.origin().bindable_event()
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

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// Either an *original* virtual slot, or a virtual slot after substitution.
    pub struct VirtualSlot: Thingy {
        fn VirtualSlot() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &SemanticHost) -> Thingy {
            self.static_type(host)
        }
    }

    pub struct OriginalVirtualSlot: VirtualSlot {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_getter: Option<Thingy> = None;
        let ref m_setter: Option<Thingy> = None;
        let ref m_static_type: Option<Thingy> = None;
        let ref m_parent: Option<Thingy> = None;
        let m_flags: VirtualSlotFlags = VirtualSlotFlags::empty();
        let ref m_bindable_event: Option<String> = None;

        pub(crate) fn OriginalVirtualSlot(name: &QName) {
            super();
            self.set_m_name(Some(name.clone()));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn getter(&self, host: &SemanticHost) -> Option<Thingy> {
            self.m_getter()
        }

        pub override fn set_getter(&self, m: Option<Thingy>) {
            self.set_m_getter(m);
        }

        pub override fn setter(&self, host: &SemanticHost) -> Option<Thingy> {
            self.m_setter()
        }

        pub override fn set_setter(&self, m: Option<Thingy>) {
            self.set_m_setter(m);
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(VirtualSlotFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(VirtualSlotFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        pub override fn read_only(&self, host: &SemanticHost) -> bool {
            self.setter(host).is_none()
        }

        pub override fn write_only(&self, host: &SemanticHost) -> bool {
            self.getter(host).is_none()
        }

        pub override fn static_type(&self, host: &SemanticHost) -> Thingy {
            if let Some(r) = self.m_static_type() {
                return r.clone();
            }

            let mut deduced_type: Option<Thingy> = None;

            // Deduce type from getter
            if let Some(getter) = self.m_getter() {
                let signature: Thingy = getter.signature(host);
                if !signature.is::<UnresolvedThingy>() {
                    deduced_type = Some(signature.result_type());
                }
            }

            // Deduce type from setter
            if let Some(setter) = self.m_setter() {
                let signature: Thingy = setter.signature(host);
                if !signature.is::<UnresolvedThingy>() {
                    deduced_type = Some(signature.params().get(0).unwrap().static_type.clone());
                }
            }

            if deduced_type.is_none() {
                return host.unresolved_thingy();
            }

            self.set_m_static_type(deduced_type.clone());
            deduced_type.unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        /// The event name indicated by a `[Bindable]` meta-data tag.
        pub override fn bindable_event(&self) -> Option<String> {
            self.m_bindable_event()
        }

        pub override fn set_bindable_event(&self, name: Option<String>) {
            self.set_m_bindable_event(name);
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

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct VirtualSlotAfterSubstitution: VirtualSlot {
        let ref m_origin: Option<Thingy> = None;
        let ref m_indirect_type_params: SharedArray<Thingy> = SharedArray::new();
        let ref m_indirect_substitute_types: SharedArray<Thingy> = SharedArray::new();
        let ref m_getter: Option<Thingy> = None;
        let ref m_setter: Option<Thingy> = None;
        let ref m_static_type: Option<Thingy> = None;

        pub(crate) fn VirtualSlotAfterSubstitution(origin: &Thingy, indirect_type_params: &SharedArray<Thingy>, indirect_substitute_types: &SharedArray<Thingy>) {
            super();
            self.set_m_origin(Some(origin.clone()));
            self.set_m_indirect_type_params(indirect_type_params.clone());
            self.set_m_indirect_substitute_types(indirect_substitute_types.clone());
        }

        pub override fn location(&self) -> Option<Location> {
            None
        }

        pub override fn origin(&self) -> Thingy {
            self.m_origin().unwrap()
        }

        pub override fn indirect_type_params(&self) -> SharedArray<Thingy> {
            self.m_indirect_type_params()
        }

        pub override fn indirect_substitute_types(&self) -> SharedArray<Thingy> {
            self.m_indirect_substitute_types()
        }

        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        pub override fn getter(&self, host: &SemanticHost) -> Option<Thingy> {
            if let Some(r) = self.m_getter() {
                return Some(r);
            }
            let r = self.origin().getter(host);
            if r.is_none() {
                return r;
            }
            let r = TypeSubstitution(host).exec(&r.unwrap(), &self.indirect_type_params(), &self.indirect_substitute_types());
            self.set_m_getter(Some(r.clone()));
            Some(r)
        }

        pub override fn setter(&self, host: &SemanticHost) -> Option<Thingy> {
            if let Some(r) = self.m_setter() {
                return Some(r);
            }
            let r = self.origin().setter(host);
            if r.is_none() {
                return r;
            }
            let r = TypeSubstitution(host).exec(&r.unwrap(), &self.indirect_type_params(), &self.indirect_substitute_types());
            self.set_m_setter(Some(r.clone()));
            Some(r)
        }

        pub override fn is_external(&self) -> bool {
            self.origin().is_external()
        }

        pub override fn read_only(&self, host: &SemanticHost) -> bool {
            self.origin().read_only(host)
        }

        pub override fn write_only(&self, host: &SemanticHost) -> bool {
            self.origin().write_only(host)
        }

        pub override fn static_type(&self, host: &SemanticHost) -> Thingy {
            if let Some(r) = self.m_static_type() {
                return r;
            }
            let r = self.origin().static_type(host);
            if r.is::<UnresolvedThingy>() {
                return r;
            }
            let r = TypeSubstitution(host).exec(&r, &self.indirect_type_params(), &self.indirect_substitute_types());
            self.set_m_static_type(Some(r.clone()));
            r
        }

        pub override fn bindable_event(&self) -> Option<String> {
            self.origin().bindable_event()
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.origin().parent()
        }

        pub override fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.origin().asdoc()
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// Either an *original* method slot, or a method slot after substitution.
    pub struct MethodSlot: Thingy {
        fn MethodSlot() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &SemanticHost) -> Thingy {
            host.function_type()
        }
    }

    pub struct OriginalMethodSlot: MethodSlot {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_activation: Option<Thingy> = None;
        let ref m_signature: Option<Thingy> = None;
        let ref m_parent: Option<Thingy> = None;
        let ref m_of_virtual_slot: Option<Thingy> = None;
        let ref m_overriden_by: SharedArray<Thingy> = SharedArray::new();
        let ref m_overrides_method: Option<Thingy> = None;
        let m_flags: MethodSlotFlags = MethodSlotFlags::empty();

        pub(crate) fn OriginalMethodSlot(name: &QName, signature: &Thingy) {
            super();
            self.set_m_name(Some(name.clone()));
            self.set_m_signature(Some(signature.clone()));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_EXTERNAL)
        }
    
        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        pub override fn is_final(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_FINAL)
        }
    
        pub override fn set_is_final(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_FINAL, value);
            self.set_m_flags(v);
        }

        pub override fn is_static(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_STATIC)
        }
    
        pub override fn set_is_static(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_STATIC, value);
            self.set_m_flags(v);
        }

        pub override fn is_abstract(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_ABSTRACT)
        }
    
        pub override fn set_is_abstract(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_ABSTRACT, value);
            self.set_m_flags(v);
        }

        pub override fn is_overriding(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_OVERRIDING)
        }
    
        pub override fn set_is_overriding(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_OVERRIDING, value);
            self.set_m_flags(v);
        }

        pub override fn is_async(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_ASYNC)
        }
    
        pub override fn set_is_async(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_ASYNC, value);
            self.set_m_flags(v);
        }

        pub override fn is_generator(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_GENERATOR)
        }
    
        pub override fn set_is_generator(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_GENERATOR, value);
            self.set_m_flags(v);
        }

        pub override fn is_constructor(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_CONSTRUCTOR)
        }
    
        pub override fn set_is_constructor(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_CONSTRUCTOR, value);
            self.set_m_flags(v);
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }
    
        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
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

        pub override fn signature(&self, host: &SemanticHost) -> Thingy {
            self.m_signature().unwrap()
        }

        pub override fn set_signature(&self, signature: &Thingy) {
            self.set_m_signature(Some(signature.clone()));
        }

        pub override fn activation(&self) -> Option<Thingy> {
            self.m_activation()
        }

        pub override fn set_activation(&self, activation: Option<Thingy>) {
            self.set_m_activation(activation);
        }

        pub override fn of_virtual_slot(&self, host: &SemanticHost) -> Option<Thingy> {
            self.m_of_virtual_slot()
        }

        pub override fn set_of_virtual_slot(&self, virtual_slot: Option<Thingy>) {
            self.set_m_of_virtual_slot(virtual_slot);
        }

        pub override fn overriden_by(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            self.m_overriden_by()
        }

        pub override fn overrides_method(&self, host: &SemanticHost) -> Option<Thingy> {
            self.m_overrides_method()
        }

        pub override fn set_overrides_method(&self, method: Option<Thingy>) {
            self.set_m_overrides_method(method);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct MethodSlotAfterSubstitution: MethodSlot {
        let ref m_origin: Option<Thingy> = None;
        let ref m_indirect_type_params: SharedArray<Thingy> = SharedArray::new();
        let ref m_indirect_substitute_types: SharedArray<Thingy> = SharedArray::new();
        let ref m_signature: Option<Thingy> = None;
        let ref m_of_virtual_slot: Option<Thingy> = None;
        let ref m_overriden_by: Option<SharedArray<Thingy>> = None;
        let ref m_overrides_method: Option<Thingy> = None;
        let m_is_overriding: bool = false;

        pub fn MethodSlotAfterSubstitution(origin: &Thingy, indirect_type_params: &SharedArray<Thingy>, indirect_substitute_types: &SharedArray<Thingy>) {
            super();
            self.set_m_origin(Some(origin.clone()));
            self.set_m_indirect_type_params(indirect_type_params.clone());
            self.set_m_indirect_substitute_types(indirect_substitute_types.clone());
        }

        pub override fn origin(&self) -> Thingy {
            self.m_origin().unwrap()
        }

        pub override fn indirect_type_params(&self) -> SharedArray<Thingy> {
            self.m_indirect_type_params()
        }

        pub override fn indirect_substitute_types(&self) -> SharedArray<Thingy> {
            self.m_indirect_substitute_types()
        }

        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        pub override fn is_external(&self) -> bool {
            self.origin().is_external()
        }

        pub override fn is_final(&self) -> bool {
            self.origin().is_final()
        }

        pub override fn is_static(&self) -> bool {
            self.origin().is_static()
        }

        pub override fn is_abstract(&self) -> bool {
            self.origin().is_abstract()
        }

        pub override fn is_overriding(&self) -> bool {
            self.m_is_overriding()
        }

        pub override fn set_is_overriding(&self, value: bool) {
            self.set_m_is_overriding(value);
        }

        pub override fn is_async(&self) -> bool {
            self.origin().is_async()
        }

        pub override fn is_generator(&self) -> bool {
            self.origin().is_generator()
        }

        pub override fn is_constructor(&self) -> bool {
            self.origin().is_constructor()
        }

        pub override fn location(&self) -> Option<Location> {
            None
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

        pub override fn signature(&self, host: &SemanticHost) -> Thingy {
            if let Some(r) = self.m_signature() {
                return r;
            }
            let r = self.origin().signature(host);
            if r.is::<UnresolvedThingy>() {
                return r.clone();
            }
            let r = TypeSubstitution(host).exec(&r, &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_signature(Some(r.clone()));
            r
        }

        pub override fn of_virtual_slot(&self, host: &SemanticHost) -> Option<Thingy> {
            if let Some(r) = self.m_of_virtual_slot() {
                return Some(r);
            }
            let r = self.origin().of_virtual_slot(host);
            if r.is_none() {
                return None;
            }
            let r = TypeSubstitution(host).exec(&r.unwrap(), &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_of_virtual_slot(Some(r.clone()));
            Some(r)
        }

        pub override fn overriden_by(&self, host: &SemanticHost) -> SharedArray<Thingy> {
            if let Some(r) = self.m_overriden_by() {
                return r;
            }
            let r = self.origin().overriden_by(host);
            let r: SharedArray<Thingy> = r.iter().map(|r| TypeSubstitution(host).exec(&r, &self.m_indirect_type_params(), &self.indirect_substitute_types())).collect();
            self.set_m_overriden_by(Some(r.clone()));
            r
        }

        pub override fn overrides_method(&self, host: &SemanticHost) -> Option<Thingy> {
            if let Some(r) = self.m_overrides_method() {
                return Some(r);
            }
            let r = self.origin().overrides_method(host);
            if r.is_none() {
                return None;
            }
            let r = TypeSubstitution(host).exec(&r.unwrap(), &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_overrides_method(Some(r.clone()));
            Some(r)
        }

        pub override fn set_overrides_method(&self, method: Option<Thingy>) {
            self.set_m_overrides_method(method);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct PackagePropertyImport: Thingy {
        let ref m_property: Option<Thingy> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn PackagePropertyImport(property: &Thingy, location: Option<Location>) {
            super();
            self.set_m_property(Some(property.clone()));
            self.set_m_location(location);
        }

        pub override fn property(&self) -> Thingy {
            self.m_property().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }
    }

    pub struct PackageWildcardImport: Thingy {
        let ref m_package: Option<Thingy> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn PackageWildcardImport(package: &Thingy, location: Option<Location>) {
            super();
            self.set_m_package(Some(package.clone()));
            self.set_m_location(location);
        }

        pub override fn package(&self) -> Thingy {
            self.m_package().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }
    }

    pub struct PackageRecursiveImport: Thingy {
        let ref m_package: Option<Thingy> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn PackageRecursiveImport(package: &Thingy, location: Option<Location>) {
            super();
            self.set_m_package(Some(package.clone()));
            self.set_m_location(location);
        }

        pub override fn package(&self) -> Thingy {
            self.m_package().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }
    }

    pub struct Scope: Thingy {
        let ref m_parent: Option<Thingy> = None;
        let ref m_properties: NameMap = NameMap::new();
        let ref m_open_ns_set: SharedArray<Thingy> = SharedArray::new();
        let ref m_import_list: SharedArray<Thingy> = SharedArray::new();

        pub(crate) fn Scope() {
            super();
        }

        pub override fn parent(&self) -> Option<Thingy> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Thingy>) {
            self.set_m_parent(p);
        }

        pub override fn properties(&self, host: &SemanticHost) -> NameMap {
            self.m_properties()
        }

        pub override fn open_ns_set(&self) -> SharedArray<Thingy> {
            self.m_open_ns_set()
        }

        /// List of [`PackagePropertyImport`], [`PackageWildcardImport`], or [`PackageRecursiveImport`].
        pub override fn import_list(&self) -> SharedArray<Thingy> {
            self.m_import_list()
        }
    }

    pub struct WithScope: Scope {
        let ref m_object: Option<Thingy> = None;

        pub(crate) fn WithScope(object: &Thingy) {
            super();
            self.set_m_object(Some(object.clone()));
        }

        pub override fn object(&self) -> Thingy {
            self.m_object().unwrap()
        }
    }

    pub struct FilterScope: Scope {
        let ref m_base: Option<Thingy> = None;

        pub(crate) fn FilterScope(base: &Thingy) {
            super();
            self.set_m_base(Some(base.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }
    }

    pub struct Activation: Scope {
        let ref m_method: Option<Thingy> = None;
        let ref m_this: Option<Thingy> = None;
        let ref m_property_has_capture: Option<SharedArray<Thingy>> = None;
        let ref m_cfg: ControlFlowGraph = ControlFlowGraph::new();

        pub(crate) fn Activation(of_method: &Thingy) {
            super();
            self.set_m_method(Some(of_method.clone()));
        }

        pub override fn of_method(&self) -> Thingy {
            self.m_method().unwrap()
        }

        /// An optional `ThisObject` value.
        pub override fn this(&self) -> Option<Thingy> {
            self.m_this()
        }

        /// Sets a `ThisObject` value.
        pub override fn set_this(&self, this: Option<Thingy>) {
            self.set_m_this(this);
        }

        /// Indicates whether an activation's property has been captured
        /// by a subsequent activation. Properties include, for example, the range from the
        /// activation to an inner most block scope.
        pub override fn property_has_capture(&self, property: &Thingy) -> bool {
            if let Some(set) = self.m_property_has_capture() {
                set.includes(property)
            } else {
                false
            }
        }

        pub override fn set_property_has_capture(&self, property: &Thingy, value: bool) {
            if let Some(mut set) = self.m_property_has_capture() {
                if value {
                    if !set.includes(property) {
                        set.push(property.clone());
                    }
                } else {
                    let i = set.index_of(property);
                    if let Some(i) = i {
                        set.remove(i);
                    }
                }
            } else if value {
                self.set_m_property_has_capture(Some(shared_array![property.clone()]));
            }
        }
    }

    pub struct ClassScope: Scope {
        let ref m_class: Option<Thingy> = None;

        pub(crate) fn ClassScope(class: &Thingy) {
            super();
            self.set_m_class(Some(class.clone()));
        }

        pub override fn class(&self) -> Thingy {
            self.m_class().unwrap()
        }
    }

    pub struct EnumScope: Scope {
        let ref m_class: Option<Thingy> = None;

        pub(crate) fn EnumScope(class: &Thingy) {
            super();
            self.set_m_class(Some(class.clone()));
        }

        pub override fn class(&self) -> Thingy {
            self.m_class().unwrap()
        }
    }

    pub struct InterfaceScope: Scope {
        let ref m_itrfc: Option<Thingy> = None;

        pub(crate) fn InterfaceScope(itrfc: &Thingy) {
            super();
            self.set_m_itrfc(Some(itrfc.clone()));
        }

        pub override fn interface(&self) -> Thingy {
            self.m_itrfc().unwrap()
        }
    }

    pub struct PackageScope: Scope {
        let ref m_pckg: Option<Thingy> = None;

        pub(crate) fn PackageScope(pckg: &Thingy) {
            super();
            self.set_m_pckg(Some(pckg.clone()));
        }

        pub override fn package(&self) -> Thingy {
            self.m_pckg().unwrap()
        }
    }

    pub struct Value: Thingy {
        let ref m_static_type: Option<Thingy> = None;

        pub(crate) fn Value(static_type: &Thingy) {
            super();
            self.set_m_static_type(Some(static_type.clone()));
        }

        pub override fn static_type(&self, host: &SemanticHost) -> Thingy {
            self.m_static_type().unwrap()
        }

        pub override fn set_static_type(&self, value: Thingy) {
            self.set_m_static_type(Some(value));
        }
    }

    pub struct Constant: Value {
        pub(crate) fn Constant(static_type: &Thingy) {
            super(static_type);
        }
    }

    /// Constant with possible types being `*` or `Object`.
    pub struct UndefinedConstant: Constant {
        pub(crate) fn UndefinedConstant(static_type: &Thingy) {
            super(static_type);
        }

        pub override fn clone_constant(&self, host: &SemanticHost) -> Thingy {
            host.factory().create_undefined_constant(&self.static_type(host))
        }
    }

    pub struct NullConstant: Constant {
        pub(crate) fn NullConstant(static_type: &Thingy) {
            super(static_type);
        }



        pub override fn clone_constant(&self, host: &SemanticHost) -> Thingy {
            host.factory().create_null_constant(&self.static_type(host))
        }
    }

    pub struct NumberConstant: Constant {
        let ref m_value: NumberVariant = NumberVariant::Int(0);

        pub(crate) fn NumberConstant(value: NumberVariant, static_type: &Thingy) {
            super(static_type);
            self.set_m_value(value);
        }

        pub override fn number_value(&self) -> NumberVariant {
            self.m_value()
        }

        pub override fn clone_constant(&self, host: &SemanticHost) -> Thingy {
            host.factory().create_number_constant(self.m_value(), &self.static_type(host))
        }
    }

    pub struct StringConstant: Constant {
        let ref m_value: String = String::new();

        pub(crate) fn StringConstant(value: String, static_type: &Thingy) {
            super(static_type);
            self.set_m_value(value);
        }

        pub override fn string_value(&self) -> String {
            self.m_value()
        }

        pub override fn clone_constant(&self, host: &SemanticHost) -> Thingy {
            host.factory().create_string_constant(self.m_value(), &self.static_type(host))
        }
    }

    pub struct BooleanConstant: Constant {
        let m_value: bool = true;

        pub(crate) fn BooleanConstant(value: bool, static_type: &Thingy) {
            super(static_type);
            self.set_m_value(value);
        }

        pub override fn boolean_value(&self) -> bool {
            self.m_value()
        }

        pub override fn clone_constant(&self, host: &SemanticHost) -> Thingy {
            host.factory().create_boolean_constant(self.m_value(), &self.static_type(host))
        }
    }

    pub struct ThisObject: Value {
        pub(crate) fn ThisObject(static_type: &Thingy) {
            super(static_type);
        }
    }

    /// The `import.meta` value.
    pub struct MetaProperty: Value {
        pub(crate) fn MetaProperty(static_type: &Thingy) {
            super(static_type);
        }
    }

    /// The `import.meta.env` value.
    pub struct MetaEnvProperty: Value {
        pub(crate) fn MetaEnvProperty(static_type: &Thingy) {
            super(static_type);
        }
    }

    pub struct ReferenceValue: Value {
        pub(crate) fn ReferenceValue(static_type: &Thingy) {
            super(static_type);
        }
    }

    pub struct TypeAsReferenceValue: ReferenceValue {
        let ref m_type: Option<Thingy> = None;

        pub(crate) fn TypeAsReferenceValue(referenced_type: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_type(Some(referenced_type.clone()));
        }

        pub override fn referenced_type(&self) -> Thingy {
            self.m_type().unwrap()
        }
    }

    pub struct XmlReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_qual: Option<Thingy> = None;
        let ref m_key: Option<Thingy> = None;

        pub(crate) fn XmlReferenceValue(base: &Thingy, qualifier: Option<Thingy>, key: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Thingy> {
            self.m_qual()
        }

        pub override fn key(&self) -> Thingy {
            self.m_key().unwrap()
        }
    }

    pub struct DynamicReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_qual: Option<Thingy> = None;
        let ref m_key: Option<Thingy> = None;

        pub(crate) fn DynamicReferenceValue(base: &Thingy, qualifier: Option<Thingy>, key: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Thingy> {
            self.m_qual()
        }

        pub override fn key(&self) -> Thingy {
            self.m_key().unwrap()
        }
    }

    pub struct StaticReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_property: Option<Thingy> = None;

        pub(crate) fn StaticReferenceValue(base: &Thingy, property: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_property(Some(property.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn property(&self) -> Thingy {
            self.m_property().unwrap()
        }
    }

    pub struct InstanceReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_property: Option<Thingy> = None;

        pub(crate) fn InstanceReferenceValue(base: &Thingy, property: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_property(Some(property.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn property(&self) -> Thingy {
            self.m_property().unwrap()
        }
    }

    pub struct TupleReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_index: usize = 0;

        pub(crate) fn TupleReferenceValue(base: &Thingy, index: usize, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_index(index);
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn tuple_index(&self) -> usize {
            self.m_index()
        }
    }

    pub struct ScopeReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_property: Option<Thingy> = None;

        pub(crate) fn ScopeReferenceValue(base: &Thingy, property: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_property(Some(property.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn property(&self) -> Thingy {
            self.m_property().unwrap()
        }
    }

    pub struct DynamicScopeReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_qual: Option<Thingy> = None;
        let ref m_key: Option<Thingy> = None;

        pub(crate) fn DynamicScopeReferenceValue(base: &Thingy, qualifier: Option<Thingy>, key: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Thingy> {
            self.m_qual()
        }

        pub override fn key(&self) -> Thingy {
            self.m_key().unwrap()
        }
    }

    pub struct PackageReferenceValue: ReferenceValue {
        let ref m_base: Option<Thingy> = None;
        let ref m_property: Option<Thingy> = None;

        pub(crate) fn PackageReferenceValue(base: &Thingy, property: &Thingy, static_type: &Thingy) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_property(Some(property.clone()));
        }

        pub override fn base(&self) -> Thingy {
            self.m_base().unwrap()
        }

        pub override fn property(&self) -> Thingy {
            self.m_property().unwrap()
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
    pub fn in_public_or_protected_ns(&self) -> bool {
        let ns = self.namespace();
        ns.is_public_ns() || ns.is_protected_ns()
    }

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
        const IS_FINAL      = 0b00000001;
        const IS_STATIC     = 0b00000010;
        const IS_ABSTRACT   = 0b00000100;
        const IS_DYNAMIC    = 0b00001000;
        const IS_OPTION_SET = 0b00010000;
        const IS_EXTERNAL   = 0b00100000;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct VariableSlotFlags: u16 {
        const IS_EXTERNAL   = 0b00000001;
        const READ_ONLY     = 0b00000010;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct VirtualSlotFlags: u16 {
        const IS_EXTERNAL   = 0b00000001;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct MethodSlotFlags: u16 {
        const IS_FINAL          = 0b000000001;
        const IS_STATIC         = 0b000000010;
        const IS_ABSTRACT       = 0b000000100;
        const IS_EXTERNAL       = 0b000001000;
        const IS_OVERRIDING     = 0b000010000;
        const IS_ASYNC          = 0b000100000;
        const IS_GENERATOR      = 0b001000000;
        const IS_CONSTRUCTOR    = 0b010000000;
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

#[derive(Clone)]
pub struct ControlFlowGraph(Rc<ControlFlowGraph1>);

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self(Rc::new(ControlFlowGraph1 {
            blocks: SharedArray::new(),
            edges: SharedArray::new(),
        }))
    }

    pub fn blocks(&self) -> SharedArray<ControlFlowBlock> {
        self.0.blocks.clone()
    }

    pub fn edges(&self) -> SharedArray<ControlFlowEdge> {
        self.0.edges.clone()
    }
}

impl std::hash::Hash for ControlFlowGraph {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for ControlFlowGraph {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ControlFlowGraph {}

struct ControlFlowGraph1 {
    blocks: SharedArray<ControlFlowBlock>,
    edges: SharedArray<ControlFlowEdge>,
}

#[derive(Clone)]
pub struct ControlFlowBlock(Rc<Vec<Rc<Directive>>>);

impl ControlFlowBlock {
    pub fn new(lines: Vec<Rc<Directive>>) -> Self {
        Self(Rc::new(lines))
    }

    pub fn lines(&self) -> Rc<Vec<Rc<Directive>>> {
        self.0.clone()
    }
}

impl std::hash::Hash for ControlFlowBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for ControlFlowBlock {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ControlFlowBlock {}

#[derive(Clone)]
pub struct ControlFlowEdge {
    pub from: ControlFlowBlock,
    pub to: ControlFlowBlock,
}