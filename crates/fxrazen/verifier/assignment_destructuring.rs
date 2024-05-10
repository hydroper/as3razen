use crate::ns::*;

pub(crate) struct AssignmentDestructuringSubverifier;

impl AssignmentDestructuringSubverifier {
    /// Verifies a pattern.
    ///
    /// `init` may be a value or an `InvalidationThingy`.
    pub fn verify_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, init: &Thingy) -> Result<(), DeferError> {
        match pattern.as_ref() {
            Expression::QualifiedIdentifier(id) =>
                Self::verify_identifier_pattern(verifier, pattern, id, init),
            Expression::ObjectInitializer(literal) =>
                Self::verify_object_pattern(verifier, pattern, literal, init),
            Expression::ArrayLiteral(literal) =>
                Self::verify_array_pattern(verifier, pattern, literal, init),
            Expression::Unary(e) => {
                if e.operator == Operator::NonNull {
                    Self::verify_non_null_pattern(verifier, pattern, e, init)
                } else {
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}