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
            Directive::LabeledStatement(lstmt) => {
                Self::verify_directive(verifier, &lstmt.substatement)
            },
            Directive::IfStatement(ifstmt) => {
                let mut any_defer = Self::verify_directive(verifier, &ifstmt.consequent).is_err();
                if let Some(alt) = &ifstmt.alternative {
                    let r = Self::verify_directive(verifier, alt).is_err();
                    any_defer = any_defer || r;
                }
                if any_defer { Err(DeferError(None)) } else { Ok(()) }
            },
            Directive::SwitchStatement(swstmt) => {
                let mut any_defer = false;
                for case in &swstmt.cases {
                    let r = Self::verify_directives(verifier, &case.directives).is_err();
                    any_defer = any_defer || r;
                }
                if any_defer { Err(DeferError(None)) } else { Ok(()) }
            },
            Directive::SwitchTypeStatement(swstmt) => {
                let mut any_defer = false;
                for case in &swstmt.cases {
                    let r = Self::verify_block(verifier, &case.block).is_err();
                    any_defer = any_defer || r;
                }
                if any_defer { Err(DeferError(None)) } else { Ok(()) }
            },
            Directive::DoStatement(dostmt) => {
                Self::verify_directive(verifier, &dostmt.body)
            },
            Directive::WhileStatement(whilestmt) => {
                Self::verify_directive(verifier, &whilestmt.body)
            },
            Directive::ForStatement(forstmt) => {
                let scope = verifier.host.lazy_node_mapping(drtv, || {
                    verifier.host.factory().create_scope()
                });
                verifier.inherit_and_enter_scope(&scope);
                let r = Self::verify_directive(verifier, &forstmt.body);
                verifier.exit_scope();
                r
            },
            Directive::ForInStatement(forstmt) => {
                let scope = verifier.host.lazy_node_mapping(drtv, || {
                    verifier.host.factory().create_scope()
                });
                verifier.inherit_and_enter_scope(&scope);
                let r = Self::verify_directive(verifier, &forstmt.body);
                verifier.exit_scope();
                r
            },
            Directive::WithStatement(withstmt) => {
                Self::verify_directive(verifier, &withstmt.body)
            },
            Directive::TryStatement(trystmt) => {
                let mut any_defer = Self::verify_block(verifier, &trystmt.block).is_err();
                for catch_clause in &trystmt.catch_clauses {
                    let r = Self::verify_block(verifier, &catch_clause.block).is_err();
                    any_defer = any_defer || r;
                }
                if let Some(finally_clause) = trystmt.finally_clause.as_ref() {
                    let r = Self::verify_block(verifier, &finally_clause.block).is_err();
                    any_defer = any_defer || r;
                }
                if any_defer { Err(DeferError(None)) } else { Ok(()) }
            },
            Directive::ConfigurationDirective(cfgdrtv) => {
                let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
                if phase == VerifierPhase::Finished {
                    return Ok(());
                }
                if Self::verify_config_subdirective(verifier, &cfgdrtv.directive).is_err() {
                    Err(DeferError(None))
                } else {
                    verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                    Ok(())
                }
            },
            Directive::ImportDirective(impdrtv) => {
                Self::verify_import_directive(verifier, drtv, impdrtv)
            },
            Directive::UseNamespaceDirective(usedrtv) => {
                let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
                if phase == VerifierPhase::Finished {
                    return Ok(());
                }
                match phase {
                    VerifierPhase::Alpha => {
                        verifier.set_drtv_phase(drtv, VerifierPhase::Beta);
                        Err(DeferError(None))
                    },
                    VerifierPhase::Beta => {
                        Self::verify_use_ns_ns(verifier, &usedrtv.expression)?;
                        verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                        Ok(())
                    },
                    _ => panic!(),
                }
            },
            Directive::IncludeDirective(incdrtv) => {
                if incdrtv.nested_directives.len() == 0 {
                    return Ok(());
                }
                let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
                if phase == VerifierPhase::Finished {
                    return Ok(());
                }
                if Self::verify_directives(verifier, &incdrtv.nested_directives).is_err() {
                    Err(DeferError(None))
                } else {
                    verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                    Ok(())
                }
            },
            Directive::NormalConfigurationDirective(cfgdrtv) =>
                Self::verify_normal_config_drtv(verifier, drtv, cfgdrtv),
            Directive::PackageConcatDirective(pckgcat) =>
                Self::verify_package_concat_drtv(verifier, drtv, pckgcat),
            _ => Ok(()),
        }
    }

    fn verify_package_concat_drtv(verifier: &mut Subverifier, drtv: &Rc<Directive>, pckgcat: &PackageConcatDirective) -> Result<(), DeferError> {
        let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
        if phase == VerifierPhase::Finished {
            return Ok(());
        }

        let host = verifier.host.clone();
        let alias_or_pckg = host.lazy_node_mapping(drtv, || {
            match &pckgcat.import_specifier {
                ImportSpecifier::Identifier(name) => {
                    let name_loc = name.1.clone();

                    // Initially unresolved if deferred;
                    // resolve any unresolved form in Beta phase.
                    let mut resolvee = host.unresolved_thingy();
                    let pckg = host.factory().create_package(pckgcat.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    let open_ns_set = verifier.scope().concat_open_ns_set_of_scope_chain();
                    match pckg.properties(&host).get_in_ns_set_or_any_public_ns(&open_ns_set, &name.0) {
                        Ok(Some(resolvee1)) => {
                            Unused(&host).mark_used(&resolvee1);
                            resolvee = resolvee1;
                        },
                        Ok(None) => {},
                        Err(AmbiguousReferenceError(name)) => {
                            verifier.add_verify_error(&name_loc, FxDiagnosticKind::AmbiguousReference, diagarg![name]);
                            resolvee = host.invalidation_thingy();
                        },
                    }

                    let Some(public_ns) = verifier.scope().search_system_ns_in_scope_chain(SystemNamespaceKind::Public) else {
                        return host.invalidation_thingy();
                    };
                    let qname = host.factory().create_qname(&public_ns, name.0.clone());
                    let mut alias = host.factory().create_alias(qname.clone(), resolvee);
                    alias.set_location(Some(drtv.location()));

                    // Define the alias, handling any conflict.
                    let mut out_names = verifier.scope().search_hoist_scope().properties(&host);
                    if let Some(prev) = out_names.get(&qname) {
                        alias = verifier.handle_definition_conflict(&prev, &alias);
                    } else {
                        out_names.set(qname, alias.clone());
                    }

                    alias
                },
                ImportSpecifier::Wildcard(_) => {
                    let pckg = host.factory().create_package(pckgcat.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    let scope = verifier.scope().search_hoist_scope();
                    if !scope.is::<PackageScope>() {
                        return host.invalidation_thingy();
                    }
                    scope.package().package_concats().push(pckg.clone());
                    pckg
                },
                ImportSpecifier::Recursive(_) => {
                    let pckg = host.factory().create_package(pckgcat.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    let scope = verifier.scope().search_hoist_scope();
                    if !scope.is::<PackageScope>() {
                        return host.invalidation_thingy();
                    }

                    let out_pckg = scope.package();

                    // Concatenate packages recursively, however
                    // ensure the packages to be concatenated are not
                    // circular.
                    if out_pckg.is_package_self_referential(&pckg) {
                        let err_loc = pckgcat.package_name[0].1.combine_with(pckgcat.package_name.last().unwrap().1.clone());
                        verifier.add_verify_error(&err_loc, FxDiagnosticKind::ConcatenatingSelfReferentialPackage, diagarg![]);
                        return host.invalidation_thingy();
                    }
                    let recursive_pckgs = pckg.list_packages_recursively();
                    scope.package().package_concats().extend(recursive_pckgs);

                    pckg
                },
            }
        });
        let resolved_alias = alias_or_pckg.is::<Alias>() && !alias_or_pckg.alias_of().is::<UnresolvedThingy>();
        if alias_or_pckg.is::<InvalidationThingy>() || resolved_alias {
            verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
            return Ok(());
        }

        match phase {
            VerifierPhase::Alpha => {
                verifier.set_drtv_phase(drtv, VerifierPhase::Beta);
                Err(DeferError(None))
            },
            // In Beta, resolve the alias, or ensure
            // the concatenated package is non-empty.
            VerifierPhase::Beta => {
                match &pckgcat.import_specifier {
                    ImportSpecifier::Identifier(name) => {
                        let name_loc = name.1.clone();
                        let pckg = host.factory().create_package(pckgcat.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                        let open_ns_set = verifier.scope().concat_open_ns_set_of_scope_chain();
                        match pckg.properties(&host).get_in_ns_set_or_any_public_ns(&open_ns_set, &name.0) {
                            Ok(Some(resolvee)) => {
                                Unused(&host).mark_used(&resolvee);
                                alias_or_pckg.set_alias_of(&resolvee);
                            },
                            Ok(None) => {
                                verifier.add_verify_error(&pckgcat.package_name[0].1.combine_with(name.1.clone()), FxDiagnosticKind::ImportOfUndefined, diagarg![
                                    format!("{}.{}", pckgcat.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join("."), name.0)]);
                                alias_or_pckg.set_alias_of(&host.invalidation_thingy());
                            },
                            Err(AmbiguousReferenceError(name)) => {
                                verifier.add_verify_error(&name_loc, FxDiagnosticKind::AmbiguousReference, diagarg![name]);
                                alias_or_pckg.set_alias_of(&host.invalidation_thingy());
                            },
                        }
                    },
                    ImportSpecifier::Wildcard(_) => {
                        // Check for empty package (including concatenations) to report a warning.
                        if alias_or_pckg.is_empty_package(&host) {
                            verifier.add_verify_error(&pckgcat.package_name[0].1.combine_with(pckgcat.package_name.last().unwrap().1.clone()),
                                FxDiagnosticKind::EmptyPackage,
                                diagarg![pckgcat.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join(".")]);
                        }
                    },
                    ImportSpecifier::Recursive(_) => {
                        // Check for empty package recursively (including concatenations) to report a warning.
                        if alias_or_pckg.is_empty_package_recursive(&host) {
                            verifier.add_verify_error(&pckgcat.package_name[0].1.combine_with(pckgcat.package_name.last().unwrap().1.clone()),
                                FxDiagnosticKind::EmptyPackage,
                                diagarg![pckgcat.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join(".")]);
                        }
                    },
                }

                verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                Ok(())
            },
            _ => panic!(),
        }
    }

    fn verify_normal_config_drtv(verifier: &mut Subverifier, drtv: &Rc<Directive>, cfgdrtv: &NormalConfigurationDirective) -> Result<(), DeferError> {
        let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
        if phase == VerifierPhase::Finished {
            return Ok(());
        }
        let host = verifier.host.clone();
        let concatenated_name = format!("{}::{}", cfgdrtv.namespace.0, cfgdrtv.constant_name.0);
        let cval = host.lazy_node_mapping(drtv, || {
            let loc = cfgdrtv.namespace.1.combine_with(cfgdrtv.constant_name.1.clone());
            if let Some(cdata) = verifier.host.config_constants().get(&concatenated_name) {
                let cval = ExpSubverifier::eval_config_constant(verifier, &loc, concatenated_name, cdata).unwrap_or(host.invalidation_thingy());
                if !(cval.is::<BooleanConstant>() || cval.is::<InvalidationThingy>()) {
                    verifier.add_verify_error(&loc, FxDiagnosticKind::NotABooleanConstant, diagarg![]);
                    return host.invalidation_thingy();
                }
                cval
            } else {
                verifier.add_verify_error(&loc, FxDiagnosticKind::CannotResolveConfigConstant, diagarg![concatenated_name.clone()]);
                host.invalidation_thingy()
            }
        });

        if cval.is::<InvalidationThingy>() || !cval.boolean_value() {
            verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
            return Ok(());
        }

        // Do not just resolve the directive; if it is a block,
        // resolve it without creating a block scope for it.
        if let Directive::Block(block) = cfgdrtv.directive.as_ref() {
            Self::verify_directives(verifier, &block.directives)
        } else {
            Self::verify_directive(verifier, &cfgdrtv.directive)
        }
    }

    fn verify_use_ns_ns(verifier: &mut Subverifier, exp: &Rc<Expression>) -> Result<(), DeferError> {
        if let Expression::Sequence(seq) = exp.as_ref() {
            Self::verify_use_ns_ns(verifier, &seq.left)?;
            Self::verify_use_ns_ns(verifier, &seq.right)?;
            return Ok(());
        }
        let Some(cval) = verifier.verify_expression(exp, &default())? else {
            return Ok(());
        };
        if !cval.is::<NamespaceConstant>() {
            verifier.add_verify_error(&exp.location(), FxDiagnosticKind::NotANamespaceConstant, diagarg![]);
            return Ok(());
        }
        let ns = cval.referenced_ns();
        verifier.scope().open_ns_set().push(ns);
        Ok(())
    }

    fn verify_import_directive(verifier: &mut Subverifier, drtv: &Rc<Directive>, impdrtv: &ImportDirective) -> Result<(), DeferError> {
        let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
        if phase == VerifierPhase::Finished {
            return Ok(());
        }
        if impdrtv.alias.is_some() {
            return Self::verify_import_alias_directive(verifier, drtv, impdrtv);
        }
        let host = verifier.host.clone();
        let imp = host.lazy_node_mapping(drtv, || {
            match &impdrtv.import_specifier {
                ImportSpecifier::Identifier(_) => {
                    // Initially unresolved import; resolve it in Beta phase.
                    host.factory().create_package_property_import(&host.unresolved_thingy(), Some(drtv.location()))
                },
                ImportSpecifier::Wildcard(_) => {
                    let pckg = host.factory().create_package(impdrtv.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    host.factory().create_package_wildcard_import(&pckg, Some(drtv.location()))
                },
                ImportSpecifier::Recursive(_) => {
                    let pckg = host.factory().create_package(impdrtv.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    host.factory().create_package_recursive_import(&pckg, Some(drtv.location()))
                },
            }
        });

        match phase {
            VerifierPhase::Alpha => {
                // Mark unused
                Unused(&verifier.host).add(&imp);

                // Contribute to import list
                verifier.scope().search_hoist_scope().import_list().push(imp);

                verifier.set_drtv_phase(drtv, VerifierPhase::Beta);
                Err(DeferError(None))
            },
            VerifierPhase::Beta => {
                match &impdrtv.import_specifier {
                    ImportSpecifier::Identifier(name) => {
                        let name_loc = name.1.clone();

                        // Resolve a property import
                        let open_ns_set = verifier.scope().concat_open_ns_set_of_scope_chain();
                        let pckg = host.factory().create_package(impdrtv.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                        match pckg.properties(&host).get_in_ns_set_or_any_public_ns(&open_ns_set, &name.0) {
                            Ok(Some(prop)) => {
                                Unused(&host).mark_used(&prop);
                                imp.set_property(&prop);
                            },
                            Ok(None) => {
                                verifier.add_verify_error(&impdrtv.package_name[0].1.combine_with(name.1.clone()), FxDiagnosticKind::ImportOfUndefined, diagarg![
                                    format!("{}.{}", impdrtv.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join("."), name.0)]);

                                imp.set_property(&host.invalidation_thingy());
                            },
                            Err(AmbiguousReferenceError(name)) => {
                                verifier.add_verify_error(&name_loc, FxDiagnosticKind::AmbiguousReference, diagarg![name]);

                                imp.set_property(&host.invalidation_thingy());
                            },
                        }
                    },
                    ImportSpecifier::Wildcard(_) => {
                        // Check for empty package (including concatenations) to report a warning.
                        if imp.package().is_empty_package(&host) {
                            verifier.add_verify_error(&impdrtv.package_name[0].1.combine_with(impdrtv.package_name.last().unwrap().1.clone()),
                                FxDiagnosticKind::EmptyPackage,
                                diagarg![impdrtv.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join(".")]);
                        }
                    },
                    ImportSpecifier::Recursive(_) => {
                        // Check for empty package, recursively, (including concatenations) to report
                        // a warning.
                        if imp.package().is_empty_package_recursive(&host) {
                            verifier.add_verify_error(&impdrtv.package_name[0].1.combine_with(impdrtv.package_name.last().unwrap().1.clone()),
                                FxDiagnosticKind::EmptyPackage,
                                diagarg![impdrtv.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join(".")]);
                        }
                    },
                }

                verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                Ok(())
            },
            _ => panic!(),
        }
    }

    fn verify_import_alias_directive(verifier: &mut Subverifier, drtv: &Rc<Directive>, impdrtv: &ImportDirective) -> Result<(), DeferError> {
        let phase = verifier.lazy_init_drtv_phase(drtv, VerifierPhase::Alpha);
        if phase == VerifierPhase::Finished {
            return Ok(());
        }
        let alias_name = impdrtv.alias.as_ref().unwrap();
        let host = verifier.host.clone();

        let internal_ns = verifier.scope().search_system_ns_in_scope_chain(SystemNamespaceKind::Internal).unwrap();
        let alias_qname = host.factory().create_qname(&internal_ns, alias_name.0.clone());

        let mut alias = host.lazy_node_mapping(drtv, || {
            let alias;
            match &impdrtv.import_specifier {
                ImportSpecifier::Identifier(_) => {
                    // Initially unresolved import; resolve it in Beta phase.
                    alias = host.factory().create_alias(alias_qname.clone(), host.unresolved_thingy());
                },
                ImportSpecifier::Wildcard(_) => {
                    let pckg = host.factory().create_package(impdrtv.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    let imp = host.factory().create_package_wildcard_import(&pckg, None);
                    alias = host.factory().create_alias(alias_qname.clone(), imp);
                },
                ImportSpecifier::Recursive(_) => {
                    let pckg = host.factory().create_package(impdrtv.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                    let imp = host.factory().create_package_recursive_import(&pckg, None);
                    alias = host.factory().create_alias(alias_qname.clone(), imp);
                },
            }
            alias.set_location(Some(alias_name.1.clone()));
            alias
        });

        if alias.is::<InvalidationThingy>() {
            verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
            return Ok(());
        }

        match phase {
            VerifierPhase::Alpha => {
                // Mark unused
                Unused(&verifier.host).add(&alias);

                // Define the alias, handling any conflict.
                let mut out_names = verifier.scope().search_hoist_scope().properties(&host);
                if let Some(prev) = out_names.get(&alias_qname) {
                    alias = verifier.handle_definition_conflict(&prev, &alias);
                    host.node_mapping().set(drtv, Some(alias));
                } else {
                    out_names.set(alias_qname, alias);
                }

                verifier.set_drtv_phase(drtv, VerifierPhase::Beta);
                Err(DeferError(None))
            },
            VerifierPhase::Beta => {
                // Resolve property or make sure an aliased package is not empty.

                match &impdrtv.import_specifier {
                    ImportSpecifier::Identifier(name) => {
                        let name_loc = name.1.clone();

                        // Resolve a property import
                        let open_ns_set = verifier.scope().concat_open_ns_set_of_scope_chain();
                        let pckg = host.factory().create_package(impdrtv.package_name.iter().map(|name| name.0.as_str()).collect::<Vec<_>>());
                        match pckg.properties(&host).get_in_ns_set_or_any_public_ns(&open_ns_set, &name.0) {
                            Ok(Some(prop)) => {
                                Unused(&host).mark_used(&prop);
                                alias.set_alias_of(&prop);
                            },
                            Ok(None) => {
                                verifier.add_verify_error(&impdrtv.package_name[0].1.combine_with(name.1.clone()), FxDiagnosticKind::ImportOfUndefined, diagarg![
                                    format!("{}.{}", impdrtv.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join("."), name.0)]);

                                alias.set_alias_of(&host.invalidation_thingy());
                            },
                            Err(AmbiguousReferenceError(name)) => {
                                verifier.add_verify_error(&name_loc, FxDiagnosticKind::AmbiguousReference, diagarg![name]);

                                alias.set_alias_of(&host.invalidation_thingy());
                            },
                        }
                    },
                    ImportSpecifier::Wildcard(_) => {
                        // Check for empty package (including concatenations) to report a warning.
                        if alias.alias_of().package().is_empty_package(&host) {
                            verifier.add_verify_error(&impdrtv.package_name[0].1.combine_with(impdrtv.package_name.last().unwrap().1.clone()),
                                FxDiagnosticKind::EmptyPackage,
                                diagarg![impdrtv.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join(".")]);
                        }
                    },
                    ImportSpecifier::Recursive(_) => {
                        // Check for empty package, recursively, (including concatenations) to report
                        // a warning.
                        if alias.alias_of().package().is_empty_package_recursive(&host) {
                            verifier.add_verify_error(&impdrtv.package_name[0].1.combine_with(impdrtv.package_name.last().unwrap().1.clone()),
                                FxDiagnosticKind::EmptyPackage,
                                diagarg![impdrtv.package_name.iter().map(|name| name.0.clone()).collect::<Vec<_>>().join(".")]);
                        }
                    },
                }

                verifier.set_drtv_phase(drtv, VerifierPhase::Finished);
                Ok(())
            },
            _ => panic!(),
        }
    }

    fn verify_config_subdirective(verifier: &mut Subverifier, drtv: &Rc<Directive>) -> Result<(), DeferError> {
        match drtv.as_ref() {
            Directive::Block(block) => {
                Self::verify_directives(verifier, &block.directives)
            },
            Directive::IfStatement(ifstmt) => {
                let Ok(cval) = verifier.verify_expression(&ifstmt.test, &default()) else {
                    verifier.add_verify_error(&ifstmt.test.location(), FxDiagnosticKind::ReachedMaximumCycles, diagarg![]);
                    return Ok(());
                };
                let Some(cval) = cval else {
                    return Ok(());
                };
                if !cval.is::<BooleanConstant>() {
                    verifier.host.node_mapping().set(&ifstmt.test, None);
                    verifier.add_verify_error(&ifstmt.test.location(), FxDiagnosticKind::NotABooleanConstant, diagarg![]);
                    return Ok(());
                }
                let bv = cval.boolean_value();
                if bv {
                    Self::verify_config_subdirective(verifier, &ifstmt.consequent)
                } else {
                    if let Some(alt) = &ifstmt.alternative {
                        Self::verify_config_subdirective(verifier, alt)
                    } else {
                        Ok(())
                    }
                }
            },
            _ => panic!(),
        }
    }

    pub fn verify_block(verifier: &mut Subverifier, block: &Rc<Block>) -> Result<(), DeferError> {
        let phase = verifier.lazy_init_block_phase(block, VerifierPhase::Alpha);
        if phase == VerifierPhase::Finished {
            return Ok(());
        }
        let host = verifier.host.clone();
        let scope = host.lazy_node_mapping(block, || {
            host.factory().create_scope()
        });
        verifier.inherit_and_enter_scope(&scope);
        let any_defer = Self::verify_directives(verifier, &block.directives).is_err();
        verifier.exit_scope();
        if any_defer {
            Err(DeferError(None))
        } else {
            verifier.set_block_phase(block, VerifierPhase::Finished);
            Ok(())
        }
    }
}