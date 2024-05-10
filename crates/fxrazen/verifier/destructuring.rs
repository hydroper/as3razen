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
// When a phase successfully ends, a `DeferError(None)` is thrown,
// except for after finishing `Omega`.
//
pub(crate) struct DestructuringDeclarationSubverifier;

impl DestructuringDeclarationSubverifier {
    // * [ ] Note 1: Remember to clear the phase entry after omega.

    /// Verifies a pattern.
    ///
    /// `init` may be a value or an `UnresolvedThingy`
    pub fn verify_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, init: &Thingy, read_only: bool, output: &mut NameMap, ns: &Thingy, parent: &Thingy) -> Result<(), DeferError> {
        match pattern.as_ref() {
            Expression::QualifiedIdentifier(id) =>
                Self::verify_identifier_pattern(verifier, pattern, id, init, read_only, output, ns, parent),
            Expression::ObjectInitializer(literal) =>
                Self::verify_object_pattern(verifier, pattern, literal, init, read_only, output, ns, parent),
            Expression::ArrayLiteral(literal) =>
                Self::verify_array_pattern(verifier, pattern, literal, init, read_only, output, ns, parent),
            Expression::Unary(e) => {
                if e.operator == Operator::NonNull {
                    Self::verify_non_null_pattern(verifier, pattern, e, init, read_only, output, ns, parent)
                } else {
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }

    fn verify_identifier_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, id: &QualifiedIdentifier, init: &Thingy, read_only: bool, output: &mut NameMap, ns: &Thingy, parent: &Thingy) -> Result<(), DeferError> {
        let mut slot = verifier.host.node_mapping().get(pattern);
        let mut slot_just_init = false;
        if slot.is_none() {
            let name = verifier.host.factory().create_qname(ns, id.to_identifier_name_or_asterisk().unwrap().0);
            let slot1 = verifier.host.factory().create_variable_slot(&name, read_only, &verifier.host.unresolved_thingy());
            slot1.set_location(Some(id.location.clone()));
            slot1.set_parent(Some(parent.clone()));

            if let Some(prev) = output.get(&name) {
                slot = Some(verifier.handle_definition_conflict(&prev, &slot1));
            } else {
                Unused(&verifier.host).add_named_entity(&slot1);
                output.set(name, slot1.clone());
                slot = Some(slot1);
            }

            verifier.host.node_mapping().set(pattern, slot.clone());

            slot_just_init = true;
        }

        let slot = slot.unwrap();
        if !slot.is::<VariableSlot>() {
            return Ok(());
        }

        let phase = verifier.phase_of_thingy.get(&slot).cloned();
        if phase.is_none() && !slot_just_init {
            return Ok(());
        }

        let phase = phase.unwrap_or(VerifierPhase::Alpha);
        verifier.phase_of_thingy.insert(slot.clone(), phase);

        match phase {
            VerifierPhase::Alpha => {
                verifier.phase_of_thingy.insert(slot.clone(), VerifierPhase::Omega);
                Err(DeferError(Some(VerifierPhase::Omega)))
            },
            VerifierPhase::Omega => {
                // Assign a type if unresolved
                if slot.static_type(&verifier.host).is::<UnresolvedThingy>() {
                    init.defer()?;
                    slot.set_static_type(init.static_type(&verifier.host).defer()?);
                }

                if init.is::<Constant>() {
                    slot.set_var_constant(Some(init.clone()));
                }

                verifier.phase_of_thingy.remove(&slot);

                Ok(())
            },
            _ => panic!(),
        }
    }

    fn verify_array_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, literal: &ArrayLiteral, init: &Thingy, read_only: bool, output: &mut NameMap, ns: &Thingy, parent: &Thingy) -> Result<(), DeferError> {
        let mut slot = verifier.host.node_mapping().get(pattern);
        let mut slot_just_init = false;
        if slot.is_none() {
            let name = verifier.host.empty_empty_qname();
            let slot1 = verifier.host.factory().create_variable_slot(&name, read_only, &verifier.host.unresolved_thingy());
            slot1.set_parent(Some(parent.clone()));
            slot = Some(slot1);
            verifier.host.node_mapping().set(pattern, slot.clone());

            slot_just_init = true;
        }

        let slot = slot.unwrap();

        let phase = verifier.phase_of_thingy.get(&slot).cloned();
        if phase.is_none() && !slot_just_init {
            return Ok(());
        }

        let phase = phase.unwrap_or(VerifierPhase::Alpha);
        verifier.phase_of_thingy.insert(slot.clone(), phase);

        match phase {
            VerifierPhase::Alpha => {
                let mut rest_loc: Option<Location> = None;
                let mut i: usize = 0;
                let mut rest_i: usize = 0;
                for elem in &literal.elements {
                    match elem {
                        Element::Expression(subpat) => {
                            if let Err(DeferError(subphase)) = Self::verify_pattern(verifier, subpat, &verifier.host.unresolved_thingy(), read_only, output, ns, parent) {
                                assert_eq!(subphase, Some(VerifierPhase::Omega));
                            }
                        },
                        Element::Rest((restpat, loc)) => {
                            if rest_loc.is_some() {
                                verifier.add_verify_error(loc, FxDiagnosticKind::UnexpectedRest, diagarg![]);
                            }
                            rest_i = i;
                            rest_loc = Some(loc.clone());
                            if let Err(DeferError(subphase)) = Self::verify_pattern(verifier, restpat, &verifier.host.unresolved_thingy(), read_only, output, ns, parent) {
                                assert_eq!(subphase, Some(VerifierPhase::Omega));
                            }
                        },
                        Element::Elision => {},
                    }
                    i += 1;
                }
                if rest_loc.is_some() && rest_i != i - 1 {
                    verifier.add_verify_error(&rest_loc.unwrap(), FxDiagnosticKind::UnexpectedRest, diagarg![]);
                }
                verifier.phase_of_thingy.insert(slot.clone(), VerifierPhase::Omega);
                Err(DeferError(Some(VerifierPhase::Omega)))
            },
            VerifierPhase::Omega => {
                todo()
            },
            _ => panic(),
        }
    }
}