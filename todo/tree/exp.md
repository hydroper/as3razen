# Expressions

* [ ] `Call`
  * [ ] Perform cast when base is a type
  * [ ] Call Function otherwise
    * [ ] If base is a method reference in a FixtureReferenceValue
      * [ ] Call with type checking (`ArgumentsSubverifier::verify`) using the property's signature.
    * [ ] If base is a reference with a function type, do type checking (`ArgumentsSubverifier::verify`).
* [ ] `WithTypeArguments`
  * [ ] Pass `context.followed_by_type_arguments = true` to base
* [ ] `Unary`
  * For negation
    * [ ] Pass `context.preceded_by_negative = true` to base
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
* [ ] `Function`