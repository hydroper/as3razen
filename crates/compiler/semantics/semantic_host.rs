use crate::ns::*;

pub struct SemanticHost {
    arena: ThingyArena,
    void_type: Thingy,
}

impl SemanticHost {
    pub fn new() -> Self {
        let arena = ThingyArena::new();
        let void_type = VoidType::new(&arena).into();
        Self {
            arena,
            void_type,
        }
    }

    #[inline(always)]
    pub fn factory(&self) -> ThingyFactory {
        ThingyFactory(self)
    }

    pub fn void_type(&self) -> Thingy {
        self.void_type.clone()
    }
}