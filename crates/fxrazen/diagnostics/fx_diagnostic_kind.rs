#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum FxDiagnosticKind {
    RemoveThisUnusedDiagnostic = 2048,
}

impl FxDiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}