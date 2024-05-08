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
    NoMatchingEnumMember = 2061,
    UnexpectedThis = 2062,
    ArrayLengthNotEqualsTupleLength = 2063,
    UnexpectedElision = 2064,
    UnexpectedArray = 2065,
    UnexpectedRest = 2066,
    UnexpectedObject = 2067,
    DynamicOptionNotSupported = 2068,
    UnknownOptionForClass = 2069,
    MustSpecifyOption = 2070,
    UnexpectedFieldName = 2071,
    UnexpectedNewBase = 2072,
    IncorrectNumArguments = 2073,
    IncorrectNumArgumentsNoMoreThan = 2074,
    UndefinedPropertyWithStaticType = 2075,
    InapplicableFilter = 2076,
    InapplicableDescendants = 2077,
    ASuperExpCanBeUsedOnlyIn = 2078,
    ASuperExpCanOnlyBeUsedInSubclasses = 2079,
    CallOnArrayType = 2080,
    CallOnNonFunction = 2081,
}

impl FxDiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}