use lazy_static::lazy_static;
use maplit::hashmap;
use crate::ns::*;

lazy_static! {
    static ref DATA: HashMap<i32, String> = hashmap! {
        // FxDiagnosticKind::K.id() => ".".into(),
    };
}

pub struct FxDiagnostic(pub Rc<Diagnostic>);

impl FxDiagnostic {
    pub fn new_syntax_error(location: &Location, kind: FxDiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) -> Diagnostic {
        let d = Diagnostic::new_syntax_error(location, DiagnosticKind::Expecting, arguments);
        d.set_custom_kind(Some(Rc::new(kind)));
        d
    }

    pub fn new_verify_error(location: &Location, kind: FxDiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) -> Diagnostic {
        let d = Diagnostic::new_verify_error(location, DiagnosticKind::Expecting, arguments);
        d.set_custom_kind(Some(Rc::new(kind)));
        d
    }

    pub fn new_warning(location: &Location, kind: FxDiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) -> Diagnostic {
        let d = Diagnostic::new_warning(location, DiagnosticKind::Expecting, arguments);
        d.set_custom_kind(Some(Rc::new(kind)));
        d
    }

    pub fn fx_kind(&self) -> Option<FxDiagnosticKind> {
        if let Some(k) = self.custom_kind() {
            if let Ok(k) = Rc::downcast(k) {
                Some(*k)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn fx_kind_eq(&self, kind: FxDiagnosticKind) -> bool {
        self.fx_kind().map(|k1| kind == k1).unwrap_or(false)
    }

    pub fn id(&self) -> i32 {
        self.fx_kind().map(|k| k.id()).unwrap_or(self.0.id())
    }

    /// Formats the diagnostic in English.
    pub fn format_english(&self) -> String {
        if self.fx_kind().is_none() {
            return self.0.format_english();
        }
        self.format_with_message(&self.format_message_english(), Some(self.id()))
    }

    pub fn format_message_english(&self) -> String {
        if self.fx_kind().is_none() {
            return self.0.format_message_english();
        }
        self.format_message(&DATA)
    }
}

impl std::ops::Deref for FxDiagnostic {
    type Target = Rc<Diagnostic>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}