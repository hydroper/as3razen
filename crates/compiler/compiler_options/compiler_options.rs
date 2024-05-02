#[derive(Clone)]
pub struct CompilerOptions {
    pub strict: bool,
    pub infer_types: bool,
    /// Whether to use block scoped properties.
    /// If this is true:
    /// * All block staements will create their own scope, and
    /// * `for..in` will contribute any bindings to a new scope
    ///   surrounding the loop's body.
    pub block_scope: bool,
    pub warnings: CompilerWarningOptions,
}

#[derive(Clone)]
#[non_exhaustive]
pub struct CompilerWarningOptions {
    pub unused: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            strict: true,
            infer_types: true,
            block_scope: true,
            warnings: Default::default(),
        }
    }
}

impl Default for CompilerWarningOptions {
    fn default() -> Self {
        Self {
            unused: true,
        }
    }
}