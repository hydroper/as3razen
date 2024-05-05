# Semantic model

* [x] QName
* [x] NumberVariant
* [x] NameMap
* [x] Things
  * [x] Namespaces
  * [x] NamespaceSet
  * [x] Packages
  * [x] Alias
  * [x] Types
  * [x] OriginalVariableSlot
  * [x] VariableSlotAfterSubstitution extends VariableSlot
  * [x] OriginalVirtualSlot extends VirtualSlot
  * [x] VirtualSlotAfterSubstitution extends VirtualSlot
  * [x] OriginalMethodSlot extends MethodSlot
  * [x] MethodSlotAfterSubstitution extends MethodSlot
  * [x] Scopes
  * [x] Values
* [ ] Interface implementation
* [ ] Method overriding
* [ ] Property lookup
  * [x] Value
  * [ ] Scope chain
    * [ ] Alias-resolvee `PackageWildcardImport` used as a qualifier in a lexical reference (scope chain) does a wildcard lookup in a package
    * [ ] Alias-resolvee `PackageRecursiveImport` used as a qualifier in a lexical reference (scope chain) does a recursive lookup in a package
    * [ ] Remove import or non `public` non `protected` entity from unused if lookup in it is successful.
  * [x] Packages
* [ ] Type conversion
  * [ ] Implicit `Function` to `function(...): E` and vice-versa
  * [ ] Convert from non `Vector` parameterized type to the same parameterized type with different type arguments
  * [ ] Covariant `Vector`
* [x] Type substitution
* [x] Type's default value