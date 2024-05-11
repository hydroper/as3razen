# Expressions

* [ ] `Function`
  * [x] Let kScope be the current scope  
  * [x] Cache the VerifierFunctionPartials of the FunctionCommon in the Subverifier.
    * [x] For the first time only, inherit scope and enter activation.
  * [x] If there is a function name
      * [x] Set the function name's self reference to a `Function` typed variable slot if not already in there.
  * [x] Verify the formal parameter list
  * [x] If the result type is specified, verify it.
  * [ ] FunctionCommon
  * [x] Set current scope to kScope.
  * [x] Return a `LambdaObject` value.