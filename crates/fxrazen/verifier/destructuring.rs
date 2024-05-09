use crate::ns::*;

// Destructuring declarations involve phase caching
// in a dictionary.
//
// Every destructuring pattern is lazily assigned a variable slot that
// is temporarily attached with a phase variant.
//
// Phases:
//
// * `Alpha` - Define variables partially, assigning their type as unresolved.
// * `Omega` - Resolve variable types based in the initialiser value.
//
// If the pattern is already assigned a variable slot and it is not
// attached with any phase variant, it is assumed to be already
// resolved. 
//
// When a phase successfully ends, a `DeferError { phase, ..default() }`
// error is thrown, except for `Omega`. The verifier
// will read that phase indicator and continue verifying, for example, other
// directives, instead of verifying again all the list just because
// of one deferred directive.
//
pub(crate) struct DestructuringDeclarationSubverifier;

impl DestructuringDeclarationSubverifier {
    // * [ ] Note 1: Remember to clear the phase entry after omega.

    pub fn verify_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, init: &Thingy, read_only: bool, output: &NameMap, ns: &Thingy, parent: &Thingy) -> Result<Option<Thingy>, DeferError> {
        //
    }
}