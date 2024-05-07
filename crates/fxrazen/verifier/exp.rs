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

    pub fn verify_null_literal(verifier: &mut Subverifier, literal: &NullLiteral, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        if let Some(t) = context.context_type.as_ref() {
            if t.includes_null(&verifier.host)? {
                return Ok(Some(verifier.host.factory().create_null_constant(t)));
            } else {
                verifier.add_verify_error(&literal.location, FxDiagnosticKind::NullNotExpectedHere, diagarg![]);
                return Ok(None);
            }
        }
        Ok(Some(verifier.host.factory().create_null_constant(&verifier.host.any_type())))
    }

    pub fn verify_boolean_literal(verifier: &mut Subverifier, literal: &BooleanLiteral, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        if let Some(t) = context.context_type.as_ref() {
            let t_esc = t.escape_of_nullable_or_non_nullable();
            if [verifier.host.any_type(), verifier.host.object_type().defer()?, verifier.host.boolean_type().defer()?].contains(&t_esc) {
                return Ok(Some(verifier.host.factory().create_boolean_constant(literal.value, &t)));
            }
        }
        Ok(Some(verifier.host.factory().create_boolean_constant(literal.value, &verifier.host.boolean_type().defer()?)))
    }

    pub fn verify_numeric_literal(verifier: &mut Subverifier, literal: &NumericLiteral, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        if let Some(t) = context.context_type.as_ref() {
            let t_esc = t.escape_of_nullable_or_non_nullable();
            if verifier.host.numeric_types()?.contains(&t_esc) {
                let n = Self::parse_number_as_data_type(&verifier.host, literal, &t_esc, context);
                if n.is_err() {
                    verifier.add_syntax_error(&literal.location, FxDiagnosticKind::CouldNotParseNumber, diagarg![t_esc]);
                    return Ok(None);
                }
                return Ok(Some(verifier.host.factory().create_number_constant(n.unwrap(), t)));
            }
        }
        let t = verifier.host.number_type().defer()?;
        let n = Self::parse_number_as_data_type(&verifier.host, literal, &t, context);
        if n.is_err() {
            verifier.add_syntax_error(&literal.location, FxDiagnosticKind::CouldNotParseNumber, diagarg![t]);
            return Ok(None);
        }
        return Ok(Some(verifier.host.factory().create_number_constant(n.unwrap(), &t)));
    }

    pub fn parse_number_as_data_type(host: &SemanticHost, literal: &NumericLiteral, data_type: &Thingy, context: &VerifierExpressionContext) -> Result<NumberVariant, ParserError> {
        if data_type == &host.number_type() {
            Ok(NumberVariant::Number(literal.parse_double(context.preceded_by_negative)?))
        } else if data_type == &host.float_type() {
            Ok(NumberVariant::Float(literal.parse_float(context.preceded_by_negative)?))
        } else if data_type == &host.int_type() {
            Ok(NumberVariant::Int(literal.parse_int(context.preceded_by_negative)?))
        } else {
            assert!(data_type == &host.uint_type());
            Ok(NumberVariant::Uint(literal.parse_uint()?))
        }
    }

    pub fn verify_string_literal(verifier: &mut Subverifier, literal: &StringLiteral, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        if let Some(t) = context.context_type.as_ref() {
            let t_esc = t.escape_of_nullable_or_non_nullable();
            if t_esc.is::<EnumType>() {
                let slot = t_esc.enum_member_slot_mapping().get(&literal.value);
                if let Some(slot) = slot {
                    let k = verifier.host.factory().create_static_reference_value(&t_esc, &slot)?;
                    return Ok(TypeConversions(&verifier.host).implicit(&k, &t, false)?);
                } else {
                    verifier.add_verify_error(&literal.location, FxDiagnosticKind::NoMatchingEnumMember, diagarg![literal.value.clone(), t_esc]);
                    return Ok(None);
                }
            }
        }
        return Ok(Some(verifier.host.factory().create_string_constant(literal.value.clone(), &verifier.host.string_type().defer()?)));
    }

    pub fn verify_this_literal(verifier: &mut Subverifier, literal: &ThisLiteral) -> Result<Option<Thingy>, DeferError> {
        let activation = verifier.scope().search_activation();
        if activation.is_some() && activation.as_ref().unwrap().this().is_some() {
            Ok(activation.clone().unwrap().this())
        } else {
            verifier.add_verify_error(&literal.location, FxDiagnosticKind::UnexpectedThis, diagarg![]);
            Ok(None)
        }
    }

    pub fn verify_reg_exp_literal(verifier: &mut Subverifier, _literal: &RegExpLiteral, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        if let Some(t) = context.context_type.as_ref() {
            let t_esc = t.escape_of_nullable_or_non_nullable();
            if [verifier.host.any_type(), verifier.host.object_type().defer()?, verifier.host.reg_exp_type().defer()?].contains(&t_esc) {
                return Ok(Some(verifier.host.factory().create_value(&t)));
            }
        }
        Ok(Some(verifier.host.factory().create_value(&verifier.host.reg_exp_type().defer()?)))
    }
}