use crate::ns::*;

pub(crate) struct ExpressionSubverifier;

impl ExpressionSubverifier {
    // QualifiedIdentifier
    pub fn verify_qualified_identifier_as_expr(verifier: &mut Subverifier, id: &QualifiedIdentifier, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        //
    }
}