use crate::ns::*;

#[derive(Clone)]
pub struct CompilerOptions {
    pub strict: bool,
    pub infer_types: bool,
    pub warnings: CompilerWarningOptions,
}

#[derive(Clone)]
#[non_exhaustive]
pub struct CompilerWarningOptions {
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            strict: true,
            infer_types: false,
            warnings: Default::default(),
        }
    }
}

impl Default for CompilerWarningOptions {
    fn default() -> Self {
    }
}