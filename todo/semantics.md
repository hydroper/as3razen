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
  * [ ] Value
    * [x] Dynamic classes (not `Proxy`; it is assumed to not be marked `dynamic`) result in untyped (dynamic) references in all cases.
    * [x] Index `Array.<T>` with type safety
    * [x] Index `Vector.<T>` with type safety
    * [x] Index with computed key
  * [ ] Scope chain
    * [ ] Concatenate open namespace sets from parent scopes
  * [ ] Packages
    * [ ] Top-level package: detect special `Vector` data type and translate it to `__AS3__.vec.Vector`
    * [ ] Package concatenations: lookup for names in the namespace set or any `public` namespace.
  * [ ] Alias-resolvee `PackageWildcardImport` used as a qualifier in a lexical reference (scope chain) does a wildcard lookup in a package
  * [ ] Alias-resolvee `PackageRecursiveImport` used as a qualifier in a lexical reference (scope chain) does a recursive lookup in a package
  * [ ] Remove import or non `public` non `protected` entity from unused if lookup in it is successful.
* [ ] Type conversion
  * [ ] Implicit `Function` to `function(...): E` and vice-versa
  * [ ] Convert from non `Vector` parameterized type to the same parameterized type with different type arguments
  * [ ] Covariant `Vector`
* [x] Type substitution
* [x] Type's default value