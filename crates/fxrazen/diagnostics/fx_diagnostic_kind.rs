#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum FxDiagnosticKind {
    EntityIsNotAType = 2048,
    ImplicitCoercionToUnrelatedType = 2049,
    EntityIsReadOnly = 2050,
    EntityIsWriteOnly = 2051,
    EntityMustNotBeDeleted = 2052,
    UndefinedProperty = 2053,
    AmbiguousReference = 2054,
    AccessOfVoid = 2055,
    AccessOfNullable = 2056,
    CouldNotExpandInlineConstant = 2057,
    ReachedMaximumCycles = 2058,
    NullNotExpectedHere = 2059,
    CouldNotParseNumber = 2060,
}

impl FxDiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}