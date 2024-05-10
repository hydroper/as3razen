# Expressions

* [ ] `Function`
  * [ ] Cache the VerifierFunctionPartials of the FunctionCommon.
  * [ ] Let kScope be the current scope
  * [ ] For the first time only, inherit scope and enter activation, otherwise set current scope to the activation.
  * [ ] Verify the formal parameter list
  * [ ] If there is a function name
    * [ ] Set the function name's self reference to a `Function` typed variable slot if not already in there.
  * [ ] If the result type is specified or the inferTypes option is off
    * [ ] If the result type is not specified, default to `*` and report a warning.
    * [ ] Construct the signature
    * [ ] Verify directives in a separate cycle context
    * [ ] Analyse the control flow
    * [ ] Ensure all code paths return a value based on the control flow graph
  * [ ] Otherwise
    * [ ] Verify directives in a separate cycle context
    * [ ] Analyse the control flow
    * [ ] Ensure all code paths return a value based on the control flow graph
    * [ ] Let the result type be the the value type returned from all code points.
      * [ ] If result types of code paths do not implicitly coerce to that of the first code path, throw a verify error.
  * [ ] Set current scope to kScope.
  * [ ] Return a `LambdaObject` value.