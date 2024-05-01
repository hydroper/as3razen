use crate::ns::*;
use smodel::smodel;

smodel! {
    type Arena = SymbolArena;

    /// Unified semantic data type representing
    /// one of several ActionScript 3 symbol variants,
    /// such as classes, variable slots, and reference values.
    pub struct Symbol {
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

        pub fn namespace(&self) -> Symbol {
            panic!();
        }

        pub fn local_name(&self) -> String {
            "".into()
        }

        fn to_string_1(&self) -> String {
            "".into()
        }
    }

    pub struct UnresolvedSymbol: Symbol {
        pub fn UnresolvedSymbol() {
            super();
        }
    }

    pub struct InvalidationSymbol: Symbol {
        pub fn InvalidationSymbol() {
            super();
        }
    }

    pub struct NamespaceSymbol: Symbol {
        pub fn NamespaceSymbol() {
            super();
        }
    }

    pub struct SystemNamespaceSymbol: NamespaceSymbol {
        let m_kind: SystemNamespaceKind = SystemNamespaceKind::Public;

        pub fn SystemNamespaceSymbol(kind: SystemNamespaceKind) {
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

    pub struct UserNamespaceSymbol: NamespaceSymbol {
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_uri: String = "".into();

        pub fn UserNamespaceSymbol(uri: String) {
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

    pub struct ExplicitNamespaceSymbol: NamespaceSymbol {
        let ref m_asdoc: Option<Rc<AsDoc>> = None;
        let ref m_uri: String = "".into();

        pub fn ExplicitNamespaceSymbol(uri: String) {
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

    pub struct QName: Symbol {
        let ref m_namespace: Option<Symbol> = None;
        let ref m_local_name: String = "".into();

        pub fn QName(namespace: Symbol, local_name: String) {
            super();
            self.set_m_namespace(Some(namespace));
            self.set_m_local_name(local_name);
        }

        pub override fn namespace(&self) -> Symbol {
            self.m_namespace().unwrap()
        }

        pub override fn local_name(&self) -> String {
            self.m_local_name()
        }

        override fn to_string_1(&self) -> String {
            let q = self.namespace();
            let ln = self.local_name();
            if q.is::<SystemNamespaceSymbol>() {
                return ln;
            }
            format!("{}::{ln}", q.uri())
        }
    }

    pub struct Type: Symbol {
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

impl ToString for Symbol {
    fn to_string(&self) -> String {
        self.to_string_1()
    }
}

impl DiagnosticArgument for Symbol {}

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