# Expressions

* [ ] `Call`
  * [ ] Perform cast when base is a type
  * [ ] Call Function otherwise
    * [ ] If base is a method reference in one of few reference values (StaticReferenceValue, InstanceReferenceValue, PackageReferenceValue, ScopeReferenceValue)
    * [ ] If base is a reference with a function type, do type checking.
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