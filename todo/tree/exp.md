# Expressions

* [ ] `Function`
  * [x] Let kScope be the current scope  
  * [x] Cache the VerifierFunctionPartials of the FunctionCommon in the Subverifier.
    * [x] For the first time only, inherit scope and enter activation.
  * [x] If there is a function name
      * [x] Set the function name's self reference to a `Function` typed variable slot if not already in there.
  * [ ] Verify the formal parameter list
  * [ ] If the result type is specified, verify it.
  * [ ] FunctionCommon
    * [ ] Set current scope to the activation.
    * [ ] If the result type is specified or the inferTypes option is off
      * [ ] If the result type is not specified, default to `*` and report a warning.
      * [ ] Construct the signature
      * [ ] Verify directives and then statements in a separate cycle context
      * [ ] Analyse the control flow
      * [ ] Ensure all code paths return a value based on the control flow graph
        * [ ] Result types that do not require a return value are `*`, `void`, `Promise.<*>`, and `Promise.<void>`.
    * [ ] Otherwise
      * [ ] Verify directives and then statements in a separate cycle context
      * [ ] Analyse the control flow
      * [ ] Ensure all code paths return a value based on the control flow graph
      * [ ] Let the result type be the the value type returned from all code points.
        * [ ] If result types of code paths do not implicitly coerce to that of the first code path, throw a verify error.
    * [ ] Cleanup the VerifierFunctionPartials cache from Subverifier.
  * [ ] Set current scope to kScope.
  * [ ] Return a `LambdaObject` value.