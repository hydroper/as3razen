# Expressions

* [ ] `Unary`
  * [ ] PostIncrement
  * [ ] PostDecrement
  * [ ] PreIncrement
  * [ ] PreDecrement
  * [ ] NonNull
  * [ ] Delete
    * [ ] Pass `context.mode = delete` to expression
  * [ ] Void
  * [ ] Typeof
  * [ ] Await
  * [ ] Yield
    * [ ] Error out indicating that yield is not supported.
  * [ ] Positive
  * [ ] Negative
    * [ ] Pass `context.preceded_by_negative = true` to base
  * [ ] BitwiseNot
  * [ ] LogicalNot
* [ ] `OptionalChaining`
* [ ] `OptionalChainingPlaceholder`
* [ ] `Binary`
* [ ] `Conditional`
* [ ] `Sequence`
* [ ] `ReservedNamespace`
* [ ] `NullableType`
* [ ] `NonNullableType`
* [ ] `AnyType`
* [ ] `VoidType`
* [ ] `ArrayType`
* [ ] `TupleType`
* [ ] `FunctionType`
* [ ] `Assignment`
  * [ ] Pass `context.mode = write` to left-hand side
* [ ] `Function`