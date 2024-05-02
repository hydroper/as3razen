use crate::ns::*;
use smodel::smodel;

smodel! {
    type Arena = ThingyArena;

    /// Unified semantic data type representing
    /// one of several ActionScript 3 variants,
    /// such as classes, variable slots, and reference values.
    pub struct Thingy {
        pub fn system_ns_kind(&self) -> Option<SystemNamespaceKind> {
            None
        }

        pub fn asdoc(&self) -> Option<Rc<AsDoc>> {
            None
        }

        pub fn set_asdoc(&self, _asdoc: Option<Rc<AsDoc>>) {}

        pub fn uri(&self) -> String {
            "".into()
        }

        pub fn ns_set_list(&self) -> SharedArray<Namespace> {
            panic!()
        }

        pub fn defer_if_unresolved(&self) -> Result<(), DeferError> {
            if self.is::<UnresolvedThingy>() {
                Err(DeferError::new())
            } else {
                Ok(())
            }
        }

        fn to_string_1(&self) -> String {
            "".into()
        }
    }

    pub struct UnresolvedThingy: Thingy {
        pub fn UnresolvedThingy() {
            super();
        }
    }

    pub struct InvalidationThingy: Thingy {
        pub fn InvalidationThingy() {
            super();
        }
    }

    pub struct Namespace: Thingy {
        pub fn Namespace() {
            super();
        }
    }

    pub struct SystemNamespace: Namespace {
        let m_kind: SystemNamespaceKind = SystemNamespaceKind::Public;

        pub fn SystemNamespace(kind: SystemNamespaceKind) {
            super();
            self.set_m_kind(kind);
        }

        pub override fn system_ns_kind(&self) -> Option<SystemNamespaceKind> {
            Some(self.m_kind())
        }

        override fn to_string_1(&self) -> String {
            self.m_kind().to_string()
        }
    }

    pub struct UserNamespace: Namespace {
        let ref m_uri: String = "".into();

        pub fn UserNamespace(uri: String) {
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

        pub fn ExplicitNamespace(uri: String) {
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

        pub fn NamespaceSet(list: SharedArray<Namespace>) {
            super();
            self.set_m_list(list);
        }

        pub override fn ns_set_list(&self) -> SharedArray<Namespace> {
            self.m_list()
        }
    }

    pub struct Type: Thingy {
    }

    pub struct VoidType : Type {
        pub fn VoidType() {
            super();
        }

        override fn to_string_1(&self) -> String {
            "void".into()
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