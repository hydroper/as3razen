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

    pub fn verify_identifier_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, id: &QualifiedIdentifier, init: &Thingy) -> Result<(), DeferError> {
        if verifier.host.node_mapping().has(pattern) || id.attribute {
            return Ok(());
        }

        init.defer()?;
        let init_st = init.static_type(&verifier.host).defer()?;

        let qn = ExpSubverifier::verify_qualified_identifier(verifier, id)?;
        if qn.is_none() {
            verifier.host.node_mapping().set(pattern, None);
            return Ok(());
        }
        let (qual, key) = qn.unwrap();

        let r = verifier.scope().lookup_in_scope_chain(&verifier.host, qual, &key);
        if r.is_err() {
            match r.unwrap_err() {
                PropertyLookupError::AmbiguousReference(name) => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AmbiguousReference, diagarg![name.clone()]);
                    verifier.host.node_mapping().set(pattern, None);
                    return Ok(());
                },
                PropertyLookupError::Defer => {
                    return Err(DeferError(None));
                },
                PropertyLookupError::VoidBase => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AccessOfVoid, diagarg![]);
                    verifier.host.node_mapping().set(pattern, None);
                    return Ok(());
                },
                PropertyLookupError::NullableObject { .. } => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AccessOfNullable, diagarg![]);
                    verifier.host.node_mapping().set(pattern, None);
                    return Ok(());
                },
            }
        }
        let r = r.unwrap();
        if r.is_none() {
            verifier.add_verify_error(&id.location, FxDiagnosticKind::UndefinedProperty, diagarg![key.local_name().unwrap()]);
            verifier.host.node_mapping().set(pattern, None);
            return Ok(());
        }
        let r = r.unwrap();

        // Mark local capture
        verifier.detect_local_capture(&r);

        // Post-processing
        let Some(val) = verifier.reference_post_processing(r, &default())? else {
            verifier.host.node_mapping().set(pattern, None);
            return Ok(());
        };

        // Implicit coercion
        let Some(val) = TypeConversions(&verifier.host).implicit(&val, &init_st, false)? else {
            verifier.add_verify_error(&id.location, FxDiagnosticKind::ImplicitCoercionToUnrelatedType, diagarg![val.static_type(&verifier.host), init_st]);
            verifier.host.node_mapping().set(pattern, None);
            return Ok(());
        };

        verifier.host.node_mapping().set(pattern, Some(val));

        Ok(())
    }

    pub fn verify_non_null_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, literal: &UnaryExpression, init: &Thingy) -> Result<(), DeferError> {
        if verifier.host.node_mapping().has(pattern) {
            return Ok(());
        }

        init.defer()?;
        init.static_type(&verifier.host).defer()?;

        let non_null_val = verifier.host.factory().create_non_null_value(&init)?;
        
        Self::verify_pattern(verifier, &literal.expression, &non_null_val)?;

        verifier.host.node_mapping().set(pattern, Some(non_null_val));

        Ok(())
    }

    pub fn verify_array_pattern(verifier: &mut Subverifier, pattern: &Rc<Expression>, literal: &ArrayLiteral, init: &Thingy) -> Result<(), DeferError> {
        if verifier.host.node_mapping().has(pattern) {
            return Ok(());
        }

        init.defer()?;
        let init_st = init.static_type(&verifier.host).defer()?;
        let init_st_esc = init_st.escape_of_non_nullable();

        // Pre verification of rest operator
        let mut rest_loc: Option<Location> = None;
        let mut i: usize = 0;
        let mut rest_i: usize = 0;
        for elem in &literal.elements {
            match elem {
                Element::Expression(_) => {},
                Element::Rest((_, loc)) => {
                    if rest_loc.is_some() {
                        verifier.add_verify_error(loc, FxDiagnosticKind::UnexpectedRest, diagarg![]);
                    }
                    rest_i = i;
                    rest_loc = Some(loc.clone());
                },
                Element::Elision => {},
            }
            i += 1;
        }
        if rest_loc.is_some() && rest_i != i - 1 {
            verifier.add_verify_error(&rest_loc.unwrap(), FxDiagnosticKind::UnexpectedRest, diagarg![]);
        }

        // Verify Vector.<T>
        if let Some(elem_type) = init_st_esc.vector_element_type(&verifier.host)? {
            Self::verify_vector_array_pattern(verifier, literal, &init_st_esc, &elem_type)?;
        // Verify Array.<T>
        } else if let Some(elem_type) = init_st_esc.array_element_type(&verifier.host)? {
            Self::verify_array_array_pattern(verifier, literal, &init_st_esc, &elem_type)?;
        // Verify tuple
        } else if init_st_esc.is::<TupleType>() {
            Self::verify_tuple_array_pattern(verifier, literal, &init_st_esc)?;
        // Verify * or Object
        } else if [verifier.host.any_type(), verifier.host.object_type().defer()?].contains(&init_st_esc) {
            Self::verify_untyped_array_pattern(verifier, literal)?;
        // Invalidation
        } else {
            Self::verify_invalidation_array_pattern(verifier, literal)?;
        }

        verifier.host.node_mapping().set(pattern, Some(init.clone()));

        Ok(())
    }

    fn verify_vector_array_pattern(verifier: &mut Subverifier, literal: &ArrayLiteral, vector_type: &Thingy, elem_type: &Thingy) -> Result<(), DeferError> {
        for elem in &literal.elements {
            match elem {
                Element::Expression(subpat) => {
                    Self::verify_pattern(verifier, subpat, &verifier.host.factory().create_value(elem_type))?;
                },
                Element::Rest((restpat, _)) => {
                    Self::verify_pattern(verifier, restpat, &verifier.host.factory().create_value(vector_type))?;
                },
                Element::Elision => {},
            }
        }
        Ok(())
    }

    fn verify_array_array_pattern(verifier: &mut Subverifier, literal: &ArrayLiteral, array_type: &Thingy, elem_type: &Thingy) -> Result<(), DeferError> {
        for elem in &literal.elements {
            match elem {
                Element::Expression(subpat) => {
                    Self::verify_pattern(verifier, subpat, &verifier.host.factory().create_value(elem_type))?;
                },
                Element::Rest((restpat, _)) => {
                    Self::verify_pattern(verifier, restpat, &verifier.host.factory().create_value(array_type))?;
                },
                Element::Elision => {},
            }
        }
        Ok(())
    }

    fn verify_tuple_array_pattern(verifier: &mut Subverifier, literal: &ArrayLiteral, tuple_type: &Thingy) -> Result<(), DeferError> {
        let elem_types = tuple_type.element_types();
        let mut i: usize = 0;
        let mut rest_found = false;

        for elem in &literal.elements {
            match elem {
                Element::Expression(subpat) => {
                    if rest_found || i >= elem_types.length() {
                        Self::verify_pattern(verifier, subpat, &verifier.host.invalidation_thingy())?;
                    } else {
                        let elem_type = elem_types.get(i).unwrap();
                        Self::verify_pattern(verifier, subpat, &verifier.host.factory().create_value(&elem_type))?;
                    }
                },
                Element::Rest((restpat, _)) => {
                    let array_type_of_any = verifier.host.array_type_of_any()?;
                    rest_found = true;
                    Self::verify_pattern(verifier, restpat, &verifier.host.factory().create_value(&array_type_of_any))?;
                },
                Element::Elision => {},
            }
            i += 1;
        }

        if i > elem_types.length() && !rest_found {
            verifier.add_verify_error(&literal.location, FxDiagnosticKind::ArrayLengthNotEqualsTupleLength, diagarg![tuple_type.clone()]);
        }

        Ok(())
    }

    fn verify_untyped_array_pattern(verifier: &mut Subverifier, literal: &ArrayLiteral) -> Result<(), DeferError> {
        for elem in &literal.elements {
            match elem {
                Element::Expression(subpat) => {
                    Self::verify_pattern(verifier, subpat, &verifier.host.factory().create_value(&verifier.host.any_type()))?;
                },
                Element::Rest((restpat, _)) => {
                    Self::verify_pattern(verifier, restpat, &verifier.host.factory().create_value(&verifier.host.any_type()))?;
                },
                Element::Elision => {},
            }
        }
        Ok(())
    }

    fn verify_invalidation_array_pattern(verifier: &mut Subverifier, literal: &ArrayLiteral) -> Result<(), DeferError> {
        for elem in &literal.elements {
            match elem {
                Element::Expression(subpat) => {
                    Self::verify_pattern(verifier, subpat, &verifier.host.invalidation_thingy())?;
                },
                Element::Rest((restpat, _)) => {
                    Self::verify_pattern(verifier, restpat, &verifier.host.invalidation_thingy())?;
                },
                Element::Elision => {},
            }
        }
        Ok(())
    }
}