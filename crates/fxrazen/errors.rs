use crate::ns::*;

/// Error used to indicate that verification must be deferred.
#[derive(Clone, Copy, PartialEq)]
pub struct DeferError();

/// Error used to indicate an ambiguous reference to a local name.
#[derive(Clone)]
#[non_exhaustive]
pub struct AmbiguousReferenceError(pub String);

#[derive(Clone)]
pub enum PropertyLookupError {
    Defer,
    AmbiguousReference(String),
    VoidBase,
    NullableObject {
        nullable_type: Thingy,
    },
}

#[derive(Clone, Copy, PartialEq)]
pub struct TypeExpectError();

#[derive(Clone)]
pub enum MethodOverridingError {
    Defer,
    MustOverrideAMethod,
    IncompatibleOverride {
        expected_signature: Thingy,
        actual_signature: Thingy,
    },
    OverridingFinalMethod,
}