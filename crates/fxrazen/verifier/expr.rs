use crate::ns::*;

pub(crate) struct ExpressionSubverifier;

impl ExpressionSubverifier {
    // QualifiedIdentifier - returns (ns, local name)
    pub fn verify_qualified_identifier(verifier: &mut Subverifier, id: &QualifiedIdentifier) -> Result<Option<(Option<Thingy>, PropertyLookupKey)>, DeferError> {
        let QualifiedIdentifier {
            qualifier,
            id,
            ..
        } = id;

        let mut failed = false;

        let mut result_qual: Option<Thingy> = None;

        if let Some(qualifier) = qualifier {
            result_qual = verifier.imp_coerce_expr(qualifier, &verifier.host.namespace_type())?;
            if result_qual.is_none() {
                failed = true;
            }
        }

        let mut result_key: Option<PropertyLookupKey> = None;

        match id {
            QualifiedIdentifierIdentifier::Id((id, _)) => {
                result_key = Some(PropertyLookupKey::LocalName(id.clone()));
            },
            QualifiedIdentifierIdentifier::Brackets(exp) => {
                let v = verifier.imp_coerce_expr(exp, &verifier.host.string_type())?;
                if let Some(v) = v {
                    result_key = Some(PropertyLookupKey::Computed(v));
                } else {
                    failed = true;
                }
            },
        }

        if failed {
            Ok(None)
        } else {
            Ok(Some((result_qual, result_key.unwrap())))
        }
    }

    // QualifiedIdentifier
    pub fn verify_qualified_identifier_as_expr(verifier: &mut Subverifier, id: &QualifiedIdentifier, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        let qn = Self::verify_qualified_identifier(verifier, id)?;
        if qn.is_none() {
            return Ok(None);
        }
        let (qual, key) = qn.unwrap();
        let r = verifier.scope().lookup_in_scope_chain(&verifier.host, qual, &key);
        if r.is_err() {
            match r.unwrap_err() {
                PropertyLookupError::AmbiguousReference(name) => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AmbiguousReference, diagarg![name.clone()]);
                    return Ok(None);
                },
                PropertyLookupError::Defer => {
                    return Err(DeferError());
                },
                PropertyLookupError::VoidBase => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AccessOfVoid, diagarg![]);
                    return Ok(None);
                },
                PropertyLookupError::NullableObject { nullable_type } => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AccessOfNullableObject, diagarg![nullable_type]);
                    return Ok(None);
                },
            }
        }
        let r = r.unwrap();
        if r.is_none() {
            verifier.add_verify_error(&id.location, FxDiagnosticKind::UndefinedProperty, diagarg![key.local_name().unwrap()]);
            return Ok(None);
        }
        let r = r.unwrap();

        if r.is::<InvalidationThingy>() {
            return Ok(None);
        }

        // Mark local capture
        if r.is::<ScopeReferenceValue>() {
            let r_act = r.base().search_activation();
            let cur_act = verifier.scope().search_activation();
            if let (Some(r_act), Some(cur_act)) = (r_act, cur_act) {
                if r_act != cur_act {
                    r_act.set_property_has_capture(&r.property(), true);
                }
            }
        }

        if r.is::<ReferenceValue>() && (r.is::<StaticReferenceValue>() || r.is::<InstanceReferenceValue>() || r.is::<ScopeReferenceValue>() || r.is::<PackageReferenceValue>()) {
            let p = r.property();

            // Auto apply parameterized types
            if (p.is::<ClassType>() || p.is::<InterfaceType>()) && p.type_params().is_some() && !context.followed_by_type_arguments {
                let mut subst = SharedArray::<Thingy>::new();
                for _ in 0..p.type_params().unwrap().length() {
                    subst.push(verifier.host.any_type());
                }
                return Ok(Some(verifier.host.factory().create_type_after_substitution(&p, &subst)));
            }

            // Compile-time constant
            if p.is::<OriginalVariableSlot>() && p.read_only(&verifier.host) && p.var_constant().is_some() {
                let r = p.var_constant().unwrap();
                return Ok(Some(r));
            }
        }

        Ok(Some(r))
    }
}