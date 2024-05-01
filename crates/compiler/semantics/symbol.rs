use crate::ns::*;
use smodel::smodel;

smodel! {
    type Arena = SymbolArena;

    /// Unified semantic data type representing
    /// one of several ActionScript 3 symbol variants,
    /// such as classes, variable slots, and reference values.
    pub struct Symbol {
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