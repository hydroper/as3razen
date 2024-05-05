# Semantic model

* [x] QName
* [x] NumberVariant
* [x] NameMap
* [ ] Things
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
  * [ ] Values
    * [x] Constants
    * [x] ThisObject
    * [x] MetaProperty
    * [x] MetaEnvProperty
    * [x] Reference values
    * [ ] ConversionValue
      * [ ] Description: Represents the resulting value of a conversion, whether implicit or explicit.
    * [ ] LambdaObject
      * [ ] Description: Represents the direct value of a `function` expression, holding back its activation.
* [ ] Interface implementation
* [ ] Method overriding
* [ ] Property resolution
  * [ ] Packages
    * [ ] Package concatenations: lookup for names in the namespace set or any `public` namespace.
  * [ ] Alias-resolvee `PackageWildcardImport` used as a qualifier in a lexical reference does a wildcard lookup in a package
  * [ ] Alias-resolvee `PackageRecursiveImport` used as a qualifier in a lexical reference does a recursive lookup in a package
  * [ ] Remove import or non `public` non `protected` entity from unused if lookup in it is successful.
* [ ] Type conversion
  * [ ] Implicit `Function` to `function(...): E` and vice-versa
* [x] Type substitution
* [ ] Type's default value