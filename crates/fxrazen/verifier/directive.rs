use crate::ns::*;

pub(crate) struct DirectiveSubverifier;

impl DirectiveSubverifier {
    pub fn verify_directives(verifier: &mut Subverifier, list: &[Rc<Directive>]) -> Result<(), DeferError> {
        let mut any_defer = false;
        for drtv in list {
            let r = Self::verify_directive(verifier, drtv).is_err();
            any_defer = any_defer || r;
        }
        if any_defer { Err(DeferError(None)) } else { Ok(()) }
    }

    pub fn verify_directive(verifier: &mut Subverifier, drtv: &Rc<Directive>) -> Result<(), DeferError> {
        match drtv.as_ref() {
            Directive::Block(block) => {
                let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
                if phase == VerifierPhase::Finished {
                    return Ok(());
                }
                let host = verifier.host.clone();
                let scope = host.lazy_node_mapping(drtv, || {
                    host.factory().create_scope()
                });
                verifier.inherit_and_enter_scope(&scope);
                let any_defer = Self::verify_directives(verifier, &block.directives).is_err();
                verifier.exit_scope();
                if any_defer {
                    Err(DeferError(None))
                } else {
                    verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}