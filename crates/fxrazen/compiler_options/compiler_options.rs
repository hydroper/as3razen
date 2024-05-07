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
    /// Whether to allow Markdown text in ASDoc comments.
    pub asdoc_markdown: bool,
    pub warnings: CompilerWarningOptions,
    /// Used for identifying the AS3 package in a MXML source tree.
    pub source_path: Vec<String>,
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
            asdoc_markdown: true,
            warnings: Default::default(),
            source_path: vec![],
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