use crate::ns::*;

pub(crate) struct ArraySubverifier;

impl ArraySubverifier {
    pub fn verify_array_literal(verifier: &mut Subverifier, literal: &ArrayLiteral, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        //
    }
}