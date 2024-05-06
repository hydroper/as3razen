#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum FxDiagnosticKind {
    EntityIsNotAType = 2048,
    ImplicitCoercionToUnrelatedType = 2049,
    EntityIsReadOnly = 2050,
    EntityIsWriteOnly = 2051,
    EntityMustNotBeDeleted = 2052,
}

impl FxDiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}