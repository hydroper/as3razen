/// Error used to indicate that verification must be deferred.
#[derive(Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct DeferError {
}

impl DeferError {
    pub fn new() -> Self {
        DeferError {}
    }
}