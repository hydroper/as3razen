use crate::ns::*;

pub struct SemanticHost {
    arena: SymbolArena,
    void_type: Symbol,
}

impl SemanticHost {
    pub fn new() -> Self {
        let arena = SymbolArena::new();
        let void_type = VoidType::new(&arena).into();
        Self {
            arena,
            void_type,
        }
    }

    #[inline(always)]
    pub fn factory(&self) -> SymbolFactory {
        SymbolFactory(self)
    }

    pub fn void_type(&self) -> Symbol {
        self.void_type.clone()
    }
}