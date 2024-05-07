use crate::ns::*;

pub(crate) struct ExpSubverifier;

impl ExpSubverifier {
    // QualifiedIdentifier - returns (ns, local name)
    pub fn verify_qualified_identifier(verifier: &mut Subverifier, id: &QualifiedIdentifier) -> Result<Option<(Option<Thingy>, PropertyLookupKey)>, DeferError> {
        let QualifiedIdentifier { qualifier, id, .. } = id;

        let mut failed = false;

        let mut result_qual: Option<Thingy> = None;

        if let Some(qualifier) = qualifier {
            result_qual = verifier.imp_coerce_exp(qualifier, &verifier.host.namespace_type())?;
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
                let v = verifier.imp_coerce_exp(exp, &verifier.host.string_type())?;
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
        // Check for inline constants
        if let Some((name, cdata)) = Self::filter_inline_constant(verifier, id) {
            // Defer
            verifier.host.string_type().defer()?;
            verifier.host.non_null_primitive_types()?;

            return Ok(Self::verify_inline_constant(verifier, &id.location, name, cdata, context));
        }

        let qn = Self::verify_qualified_identifier(verifier, id)?;
        if qn.is_none() {
            return Ok(None);
        }
        let (qual, key) = qn.unwrap();

        // Attribute
        if id.attribute {
            return Ok(Some(verifier.host.factory().create_dynamic_scope_reference_value(&verifier.scope(), qual, &key.computed_or_local_name(&verifier.host)?)));
        }

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
                PropertyLookupError::NullableObject { .. } => {
                    verifier.add_verify_error(&id.location, FxDiagnosticKind::AccessOfNullable, diagarg![]);
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

    fn filter_inline_constant(verifier: &mut Subverifier, id: &QualifiedIdentifier) -> Option<(String, String)> {
        let QualifiedIdentifier { qualifier, id, .. } = id;

        if let Some(qualifier) = qualifier {
            // Detect any inline constant
            let inlinekqid = qualifier.to_identifier_name().map(|name| name.0);
            let inlinekln = if let QualifiedIdentifierIdentifier::Id((name, _)) = id { Some(name) } else { None };
            if let (Some(inlinekqid), Some(inlinekln)) = (inlinekqid, inlinekln) {
                let inlinekid = format!("{}::{}", inlinekqid, inlinekln);
                if let Some(cdata) = verifier.host.config_constants().get(&inlinekid) {
                    return Some((inlinekid, cdata));
                }
            }
        }

        None
    }

    fn verify_inline_constant(verifier: &mut Subverifier, location: &Location, name: String, mut cdata: String, context: &VerifierExpressionContext) -> Option<Thingy> {
        cdata = cdata.trim().to_owned();

        if ["true", "false"].contains(&cdata.as_str()) {
            let boolean_type = verifier.host.boolean_type();
            if boolean_type.is::<UnresolvedThingy>() {
                verifier.add_verify_error(location, FxDiagnosticKind::CouldNotExpandInlineConstant, diagarg![]);
                return None;
            }
            return Some(verifier.host.factory().create_boolean_constant(cdata == "true", &boolean_type));
        }

        // Cache compilation unit for less memory usage
        let cu: Rc<CompilationUnit>;
        if let Some(cu1) = verifier.host.config_constants_cu().get(&name) {
            cu = cu1;
        } else {
            cu = CompilationUnit::new(None, cdata);
            verifier.host.config_constants_cu().set(name, cu.clone());
        }

        // An expression is always built for the inline constant,
        // which must be a compile-time constant.
        let exp = ParserFacade(&cu, ParserOptions::default()).parse_expression();
        if cu.invalidated() {
            verifier.add_verify_error(location, FxDiagnosticKind::CouldNotExpandInlineConstant, diagarg![]);
            return None;
        }
        let Ok(cval) = verifier.verify_expression(&exp, context) else {
            verifier.add_verify_error(location, FxDiagnosticKind::CouldNotExpandInlineConstant, diagarg![]);
            return None;
        };
        if let Some(cval) = cval.as_ref() {
            if !cval.is::<Constant>() {
                verifier.add_verify_error(location, FxDiagnosticKind::CouldNotExpandInlineConstant, diagarg![]);
                return None;
            }
        }
        cval
    }
}