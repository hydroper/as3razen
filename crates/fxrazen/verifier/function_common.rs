use crate::ns::*;

#[derive(Clone)]
pub(crate) struct VerifierFunctionPartials(Rc<VerifierFunctionPartials1>);

impl VerifierFunctionPartials {
    pub fn new(activation: &Thingy, name_span: &Location) -> Self {
        Self(Rc::new(VerifierFunctionPartials1 {
            name_span: name_span.clone(),
            activation: activation.clone(),
            params: RefCell::new(None),
            result_type: RefCell::new(None),
            signature: RefCell::new(None),
            // directives_finished: Cell::new(false),
        }))
    }

    pub fn activation(&self) -> Thingy {
        self.0.activation.clone()
    }

    pub fn name_span(&self) -> Location {
        self.0.name_span.clone()
    }

    pub fn params(&self) -> std::cell::Ref<Option<Vec<Rc<SemanticFunctionTypeParameter>>>> {
        self.0.params.borrow()
    }

    pub fn set_params(&self, list: Option<Vec<Rc<SemanticFunctionTypeParameter>>>) {
        self.0.params.replace(list);
    }

    pub fn result_type(&self) -> Option<Thingy> {
        self.0.result_type.borrow().as_ref().cloned()
    }

    pub fn set_result_type(&self, thingy: Option<Thingy>) {
        self.0.result_type.replace(thingy);
    }

    pub fn signature(&self) -> Option<Thingy> {
        self.0.signature.borrow().as_ref().cloned()
    }

    pub fn set_signature(&self, thingy: Option<Thingy>) {
        self.0.signature.replace(thingy);
    }

    /*
    pub fn directives_finished(&self) -> bool {
        self.0.directives_finished.get()
    }

    pub fn set_directives_finished(&self, value: bool) {
        self.0.directives_finished.set(value);
    }
    */
}

struct VerifierFunctionPartials1 {
    pub activation: Thingy,
    pub name_span: Location,
    pub params: RefCell<Option<Vec<Rc<SemanticFunctionTypeParameter>>>>,
    pub result_type: RefCell<Option<Thingy>>,
    pub signature: RefCell<Option<Thingy>>,
    // pub directives_finished: Cell<bool>,
}

pub(crate) struct FunctionCommonSubverifier;

impl FunctionCommonSubverifier {
    pub fn verify_function_exp_common(verifier: &mut Subverifier, common: &Rc<FunctionCommon>, partials: &VerifierFunctionPartials) -> Result<(), DeferError> {
        let host = verifier.host.clone();
        let activation =  partials.activation();
        let method = activation.of_method();
        verifier.set_scope(&activation);

        let name_span = partials.name_span();

        // Attempt to create signature
        let mut signature: Option<Thingy> = None;
        if partials.signature().is_none() && partials.result_type().is_some() {
            let mut result_type = partials.result_type().unwrap(); 

            if common.contains_await && !result_type.promise_result_type(&host)?.is_some() {
                verifier.add_verify_error(&name_span, FxDiagnosticKind::ReturnTypeDeclarationMustBePromise, diagarg![]);
                result_type = host.promise_type().defer()?.type_substitution(&host, &host.promise_type().defer()?.type_params().unwrap(), &shared_array![host.invalidation_thingy()])
            }

            let signature1 = host.factory().create_function_type(partials.params().as_ref().unwrap().clone(), result_type);
            partials.set_signature(Some(signature1.clone()));
            signature = Some(signature1);
        }

        // Set the activation method's signature to the last obtained signature if any.
        if let Some(signature) = signature.clone() {
            method.set_signature(&signature);
        }

        // Resolve directives and then statements, or just the expression body.
        match &common.body {
            Some(FunctionBody::Block(block)) => {
                let mut block_scope = host.factory().create_scope();
                verifier.inherit_and_enter_scope(&block_scope);
                DirectiveSubverifier::verify_directives(&block.directives)?;
                StatementSubverifier::verify_statements(&block.directives);
                verifier.exit_scope();
            },
            Some(FunctionBody::Expression(exp)) => {
                if let Some(result_type) = partials.result_type() {
                    verifier.imp_coerce_exp(exp, &result_type)?;
                } else {
                    verifier.verify_expression(exp, &default())?;
                }
            },
            None => {},
        }

        // Analyse the control flow (for block only).
        if let Some(FunctionBody::Block(block)) = &common.body {
            ControlFlowAnalyser::analyse_directives(&block.directives, &activation.control_flow_graph(), &mut vec![]);
        }

        // If the signature is fully resolved, ensure all code paths return a value.
        // Result types that do not require a return value are
        // `*`, `void`, `Promise.<*>`, and `Promise.<void>`.
        if let Some(signature) = partials.signature() {
            todo_here();
        // If the signature is not fully resolved due to unknown result type,
        // collect the result value types returned from all code paths,
        // ensure the result of all code paths implicitly coerce to the first code path's
        // result's type, and construct the signature into the signature local.
        //
        // If the result type does not match a Promise for an asynchronous method,
        // change it to Promise.<INVALIDATED> and report an error.
        } else {
            todo_here();
        }

        // Set the activation method's signature to the last obtained signature. 
        method.set_signature(&signature.unwrap());

        // Cleanup the VerifierFunctionPartials cache from Subverifier.
        verifier.deferred_function_exp.remove(&NodeAsKey(common.clone()));

        Ok(())
    }
}