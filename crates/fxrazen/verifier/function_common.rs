use crate::ns::*;

#[derive(Clone)]
pub(crate) struct VerifierFunctionPartials(Rc<VerifierFunctionPartials1>);

impl VerifierFunctionPartials {
    pub fn new(activation: &Thingy) -> Self {
        Self(Rc::new(VerifierFunctionPartials1 {
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

        // Attempt to create signature
        let mut signature: Option<Thingy> = None;
        if partials.signature().is_none() && partials.result_type().is_some() {
            let signature1 = host.factory().create_function_type(partials.params().as_ref().unwrap().clone(), partials.result_type().unwrap());
            partials.set_signature(Some(signature1.clone()));
            signature = Some(signature1);
        }

        // Set the activation method's signature to the last obtained signature if any.
        if let Some(signature) = signature.clone() {
            method.set_signature(&signature);
        }

        // Resolve directives and then statements.
        todo_here();

        // If the signature is fully resolved, analyse the control flow,
        // and ensure all code paths return a value.
        if let Some(signature) = partials.signature() {
            todo_here();
        // If the signature is not fully resolved due to unknown result type,
        // .
        } else {
            todo_here();
        }

        todo_here()
    }
}