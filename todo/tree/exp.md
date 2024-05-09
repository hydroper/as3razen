# Expressions

* [ ] `Unary`
  * [x] PostIncrement
  * [x] PostDecrement
  * [x] PreIncrement
  * [x] PreDecrement
  * [x] NonNull
  * [x] Delete
    * [x] Pass `context.mode = delete` to expression
  * [x] Void
  * [x] Typeof
  * [x] Await
  * [x] Yield
    * [x] Error out indicating that yield is not supported.
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
  * [ ] Applying it on a non-null primitive type results into the same type as-is without the non-nullable modifier.
* [ ] `AnyType`
* [ ] `VoidType`
* [ ] `ArrayType`
* [ ] `TupleType`
* [ ] `FunctionType`
* [ ] `Assignment`
  * [ ] Destructuring assignment (`{}, [], []!, {}!`)
  * [ ] Pass `context.mode = write` to left-hand side
* [ ] `Function`