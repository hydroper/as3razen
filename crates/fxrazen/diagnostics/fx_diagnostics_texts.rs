use lazy_static::lazy_static;
use maplit::hashmap;
use crate::ns::*;

lazy_static! {
    pub static ref DATA: HashMap<i32, String> = hashmap! {
        // FxDiagnosticKind::K.id() => ".".into(),
        FxDiagnosticKind::EntityIsNotAType.id() => "Entity is not a type.".into(),
        FxDiagnosticKind::ImplicitCoercionToUnrelatedType.id() => "Implicit coercion of a value of type {1} to an unrelated type {2}.".into(),
        FxDiagnosticKind::EntityIsReadOnly.id() => "Entity is read-only.".into(),
        FxDiagnosticKind::EntityIsWriteOnly.id() => "Entity is write-only.".into(),
        FxDiagnosticKind::EntityMustNotBeDeleted.id() => "Entity must not be deleted.".into(),
        FxDiagnosticKind::UndefinedProperty.id() => "Access of possibly undefined property '{1}'.".into(),
        FxDiagnosticKind::AmbiguousReference.id() => "Ambiguous reference to '{1}'.".into(),
        FxDiagnosticKind::AccessOfVoid.id() => "Accessing property of void.".into(),
        FxDiagnosticKind::AccessOfNullable.id() => "Accessing property of nullable data type.".into(),
        FxDiagnosticKind::CouldNotExpandInlineConstant.id() => "Could not expand inline constant.".into(),
        FxDiagnosticKind::ReachedMaximumCycles.id() => "Reached maximum cycles.".into(),
    };
}