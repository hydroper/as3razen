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
        init.defer()?;
        let init_st = init.static_type(&verifier.host).defer()?;

        if verifier.host.node_mapping().has(pattern) || id.attribute {
            return Ok(());
        }

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
        init.defer()?;
        init.static_type(&verifier.host).defer()?;

        if verifier.host.node_mapping().has(pattern) {
            return Ok(());
        }

        let non_null_val = verifier.host.factory().create_non_null_value(&init)?;
        
        Self::verify_pattern(verifier, &literal.expression, &non_null_val)?;

        verifier.host.node_mapping().set(pattern, Some(non_null_val));

        Ok(())
    }
}