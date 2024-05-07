use crate::ns::*;

/// ActionScript 3 and MXML verifier.
///
/// `Verifier` performs type checking and maps nodes to something in
/// the semantic model.
///
/// # Verifying
/// 
/// A set of programs can be verified by invoking `verify_programs()`:
/// 
/// ```ignore
/// verifier.verify_programs(program_list, mxml_list);
/// ```
/// 
/// A single expression can be verified by invoking `verify_expression()`:
/// 
/// ```ignore
/// verifier.verify_expression(&expression, Some(context_type));
/// ```
/// 
/// # Scopes
/// 
/// Enter and exit scopes by invoking `enter_scope()` and `exit_scope()` respectively.
/// Such methods may alter the `parent()` field of the scope to use the enclosing
/// scope as the parent.
///
/// ```
/// verifier.enter_scope(&scope);
/// verifier.exit_scope();
/// ```
///
/// # Symbol solving
///
/// As programs are verified, the `host.node_mapping()` object is filled
/// with mappings from program nodes to something in the semantic model.
/// 
/// ```ignore
/// // expression: Rc<Expression>
/// let thingy: Option<Thingy> = host.node_mapping().get(&expression);
/// ```
pub struct Verifier {
    verifier: Subverifier,
}

impl Verifier {
    pub(crate) const MAX_CYCLES: usize = 512;

    pub fn new(host: &Rc<SemanticHost>) -> Self {
        Self {
            verifier: Subverifier {
                host: host.clone(),
                deferred_directives: vec![],
                deferred_function_commons: vec![],
                invalidated: false,
                // deferred_counter: 0,
                scope: None,
            },
        }
    }

    /// Indicates whether an error was found while verifying the program.
    pub fn invalidated(&self) -> bool {
        self.verifier.invalidated
    }

    /// # Panics
    ///
    /// Panics if the verifier is already invalidated before verifying.
    pub fn verify_programs(&mut self, programs: Vec<Rc<Program>>, mxml_list: Vec<Rc<Mxml>>) {
        if self.verifier.invalidated {
            panic!("Verifier already invalidated.");
        }
        self.verifier.reset_state();

        todo_here();
    }

    /// Verifies an expression. Returns `None` if verification failed.
    ///
    /// # Panics
    ///
    /// Panics if the verifier is already invalidated before verifying.
    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context: &VerifierExpressionContext) -> Option<Thingy> {
        if self.verifier.invalidated {
            panic!("Verifier already invalidated.");
        }
        self.verifier.reset_state();

        let v = self.verifier.verify_expression(exp, context);
        if let Ok(v) = v {
            return v;
        }

        self.verifier.add_verify_error(&exp.location(), FxDiagnosticKind::ReachedMaximumCycles, diagarg![]);
        None
    }

    pub fn enter_scope(&mut self, scope: &Thingy) {
        self.verifier.enter_scope(scope);
    }

    pub fn exit_scope(&mut self) {
        self.verifier.exit_scope();
    }
}

pub(crate) struct Subverifier {
    pub host: Rc<SemanticHost>,
    /// List of (phase, scope, directive).
    pub deferred_directives: Vec<(usize, Thingy, Rc<Directive>)>,
    /// List of (phase, scope, common).
    pub deferred_function_commons: Vec<(usize, Thingy, Rc<FunctionCommon>)>,
    invalidated: bool,
    // pub deferred_counter: usize,
    pub scope: Option<Thingy>,
}

impl Subverifier {
    #[inline(always)]
    pub fn node_mapping(&self) -> &TreeSemantics<Thingy> {
        &self.host.node_mapping()
    }

    /// Indicates whether an error was found in the program while
    /// verifying.
    pub fn invalidated(&self) -> bool {
        self.invalidated
    }

    fn reset_state(&mut self) {
        // self.deferred_counter = 0;
        self.deferred_directives.clear();
        self.deferred_function_commons.clear();
    }

    pub fn add_syntax_error(&mut self, location: &Location, kind: FxDiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) {
        location.compilation_unit().add_diagnostic(FxDiagnostic::new_syntax_error(location, kind, arguments));
        self.invalidated = true;
    }

    pub fn add_verify_error(&mut self, location: &Location, kind: FxDiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) {
        location.compilation_unit().add_diagnostic(FxDiagnostic::new_verify_error(location, kind, arguments));
        self.invalidated = true;
    }

    pub fn add_warning(&mut self, location: &Location, kind: FxDiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) {
        location.compilation_unit().add_diagnostic(FxDiagnostic::new_warning(location, kind, arguments));
    }

    pub fn enter_scope(&mut self, scope: &Thingy) {
        let k = self.scope.clone();
        self.scope = Some(scope.clone());
        if scope.parent().is_none() {
            scope.set_parent(k);
        }
    }

    pub fn exit_scope(&mut self) {
        self.scope = self.scope.as_ref().and_then(|scope| scope.parent());
    }

    pub fn scope(&self) -> Thingy {
        self.scope.as_ref().unwrap().clone()
    }

    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context: &VerifierExpressionContext) -> Result<Option<Thingy>, DeferError> {
        let pre_result = self.host.node_mapping().get(exp);
        if let Some(pre_result) = pre_result {
            return Ok(Some(pre_result));
        }
        let result: Option<Thingy>;
        match exp.as_ref() {
            Expression::QualifiedIdentifier(id) => {
                result = ExpSubverifier::verify_qualified_identifier_as_expr(self, id, context)?;
            },
            Expression::NumericLiteral(e) => {
                result = ExpSubverifier::verify_numeric_literal(self, e, context)?;
            },
            Expression::StringLiteral(e) => {
                result = ExpSubverifier::verify_string_literal(self, e, context)?;
            },
            Expression::Paren(e) => {
                result = self.verify_expression(&e.expression, context)?;
            },
            Expression::NullLiteral(e) => {
                result = ExpSubverifier::verify_null_literal(self, e, context)?;
            },
            Expression::BooleanLiteral(e) => {
                result = ExpSubverifier::verify_boolean_literal(self, e, context)?;
            },
            Expression::ThisLiteral(e) => {
                result = ExpSubverifier::verify_this_literal(self, e)?;
            },
            Expression::RegExpLiteral(e) => {
                result = ExpSubverifier::verify_reg_exp_literal(self, e, context)?;
            },
            Expression::Invalidated(_) => {
                result = None;
            },
        }

        self.host.node_mapping().set(exp, result.clone());

        if result.is_none() {
            return Ok(result);
        }
        let result = result.unwrap();

        match context.mode {
            VerifyMode::Read => {
                if result.write_only(&self.host) {
                    self.add_verify_error(&exp.location(), FxDiagnosticKind::EntityIsWriteOnly, diagarg![]);
                }
            },
            VerifyMode::Write => {
                if result.read_only(&self.host) {
                    self.add_verify_error(&exp.location(), FxDiagnosticKind::EntityIsReadOnly, diagarg![]);
                }
            },
            VerifyMode::Delete => {
                if !result.deletable(&self.host) {
                    self.add_verify_error(&exp.location(), FxDiagnosticKind::EntityMustNotBeDeleted, diagarg![]);
                }
            },
        }

        Ok(Some(result))
    }

    pub fn verify_type_expression(&mut self, exp: &Rc<Expression>) -> Result<Option<Thingy>, DeferError> {
        let v = self.verify_expression(exp, &VerifierExpressionContext { ..default() })?;
        if v.is_none() {
            return Ok(None);
        }
        let v = v.unwrap();
        let v = v.expect_type();
        if v.is_err() {
            self.add_verify_error(&exp.location(), FxDiagnosticKind::EntityIsNotAType, diagarg![]);
            self.host.node_mapping().set(exp, None);
            return Ok(None);
        }
        let v = v.unwrap();
        self.host.node_mapping().set(exp, Some(v.clone()));
        Ok(Some(v))
    }

    /// Implicitly coerce expression to a type.
    pub fn imp_coerce_exp(&mut self, exp: &Rc<Expression>, target_type: &Thingy) -> Result<Option<Thingy>, DeferError> {
        let v = self.verify_expression(exp, &VerifierExpressionContext {
            context_type: Some(target_type.clone()),
            ..default()
        })?;
        if v.is_none() {
            return Ok(None);
        }
        let v = v.unwrap();
        let got_type = v.static_type(&self.host);
        let v = TypeConversions(&self.host).implicit(&v, target_type, false)?;
        if v.is_none() {
            self.add_verify_error(&exp.location(), FxDiagnosticKind::ImplicitCoercionToUnrelatedType, diagarg![got_type, target_type.clone()]);
            self.host.node_mapping().set(exp, None);
            return Ok(None);
        }
        let v = v.unwrap();
        self.host.node_mapping().set(exp, Some(v.clone()));
        Ok(Some(v))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VerifyMode {
    Read,
    Write,
    Delete,
}

#[derive(Clone)]
pub struct VerifierExpressionContext {
    pub context_type: Option<Thingy>,
    pub followed_by_type_arguments: bool,
    pub mode: VerifyMode,
    pub preceded_by_negative: bool,
}

impl Default for VerifierExpressionContext {
    fn default() -> Self {
        Self {
            context_type: None,
            followed_by_type_arguments: false,
            mode: VerifyMode::Read,
            preceded_by_negative: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ns::*;

    #[test]
    fn test_exp() {
        // Prepare the host
        let host = Rc::new(SemanticHost::new(SemanticHostOptions::default()));
        host.config_constants().set("FOO::X".into(), "true".into());
        
        /*
        let boolean_class = host.factory().create_class_type(
            host.factory().create_qname(&host.top_level_package().public_ns().unwrap(), "Boolean".into()),
            &host.top_level_package().public_ns().unwrap());
        host.top_level_package().properties(&host).set(boolean_class.name(), boolean_class);
        */

        let cu = CompilationUnit::new(None, "FOO::X".into());

        // Parsing
        let exp = ParserFacade(&cu, ParserOptions::default()).parse_expression();

        // Verification
        let mut verifier = Verifier::new(&host);
        let scope = host.factory().create_scope();
        scope.import_list().push(host.factory().create_package_wildcard_import(&host.top_level_package(), None));
        verifier.enter_scope(&scope);
        let _ = verifier.verify_expression(&exp, &VerifierExpressionContext {
            ..default()
        });
        /*
        assert!(cv.is_some());
        let Some(cv) = cv else { panic!(); };
        assert!(cv.is::<BooleanConstant>());
        assert!(cv.boolean_value());
        */
        verifier.exit_scope();

        // Report diagnostics
        cu.sort_diagnostics();
        for diag in cu.nested_diagnostics() {
            println!("{}", FxDiagnostic(&diag).format_english());
        }
    }
}