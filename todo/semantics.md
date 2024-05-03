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
  * [ ] Scopes
    * [ ] Field: Parent
    * [ ] Field: Open namespace set
    * [ ] Field: Package property imports
      * [ ] Description: Mapping from package to PackagePropertyImport
        * [ ] A PackagePropertyImport holds a reference to a package's property and the location of an `import ns.x;` directive.
    * [ ] Field: Package wildcard imports
      * [ ] Description: List of PackageWildcardImport
        * [ ] A PackageWildcardImport holds a reference to a package and the location of an `import ns.*;` directive.
    * [ ] WithScope
    * [ ] FilterOperatorScope
    * [ ] ActivationScope
    * [ ] ClassScope
    * [ ] EnumScope
    * [ ] InterfaceScope
    * [ ] PackageScope
  * [ ] Values
    * [ ] Constants
      * [ ] UndefinedConstant
      * [ ] NullConstant
      * [ ] NumberConstant
      * [ ] StringConstant
      * [ ] BooleanConstant
      * [ ] EnumConstant
    * [ ] ThisObject
    * [ ] MetaProperty
      * [ ] Description: Represents the `import.meta` property.
    * [ ] Reference values
      * [ ] TypeAsReferenceValue
      * [ ] XmlReferenceValue
      * [ ] DynamicReferenceValue
      * [ ] StaticReferenceValue
      * [ ] InstanceReferenceValue
      * [ ] TupleReferenceValue
      * [ ] ScopeReferenceValue
      * [ ] DynamicScopeReferenceValue
      * [ ] PackageReferenceValue
    * [ ] ConversionValue
      * [ ] Description: Represents the resulting value of a conversion, whether implicit or explicit.
    * [ ] LambdaObject
      * [ ] Description: Represents the direct value of a `function` expression, holding back its activation.
* [ ] Thingy factory
* [ ] Interface implementation
* [ ] Method overriding
* [ ] Property resolution
* [ ] Type conversion
* [x] Type substitution
* [ ] Semantic host