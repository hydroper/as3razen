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
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_uri: String = "".into();

        pub fn UserNamespace(uri: String) {
            super();
            self.set_m_uri(uri);
        }

        pub override fn uri(&self) -> String {
            self.m_uri()
        }

        pub fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        override fn to_string_1(&self) -> String {
            self.m_uri()
        }
    }

    pub struct ExplicitNamespace: Namespace {
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_uri: String = "".into();

        pub fn ExplicitNamespace(uri: String) {
            super();
            self.set_m_uri(uri);
        }

        pub override fn uri(&self) -> String {
            self.m_uri()
        }

        pub fn asdoc(&self) -> Option<Rc<AsDoc>> {
            self.m_asdoc()
        }

        pub fn set_asdoc(&self, asdoc: Option<Rc<AsDoc>>) {
            self.set_m_asdoc(asdoc);
        }

        override fn to_string_1(&self) -> String {
            self.m_uri()
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

pub struct QName {
    m_namespace: Thingy,
    m_local_name: String,
}

impl QName {
    pub fn new(namespace: Thingy, local_name: String) -> Self {
        Self {
            m_namespace: namespace,
            m_local_name: local_name,
        }
    }

    pub fn namespace(&self) -> Thingy {
        self.m_namespace.clone()
    }

    pub fn local_name(&self) -> String {
        self.m_local_name.clone()
    }
}

impl ToString for QName {
    fn to_string(&self) -> String {
        let q = self.namespace();
        let ln = self.local_name();
        if q.is::<SystemNamespace>() {
            return ln;
        }
        format!("{}::{ln}", q.uri())
    }
}

impl DiagnosticArgument for QName {}