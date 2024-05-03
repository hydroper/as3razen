# Semantic model

* [x] QName
* [x] NumberVariant
* [x] NameMap
* [ ] Warning: Entities, including classes, aliases, variable slots and so on, all must have an optional "location" field pointing to where they were defined in the source code.
  * [x] Alias
  * [x] Class
  * [x] Enum
  * [x] Interface
  * [x] Type parameter type
  * [x] Variable slot
  * [x] Virtual slot
  * [ ] Method slot
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
  * [ ] OriginalMethodSlot extends MethodSlot
    * [ ] Override `name()`
    * [ ] Override `location()`
    * [ ] Override `set_location()`
    * [ ] Override `asdoc()`
    * [ ] Override `set_asdoc()`
    * [ ] Override `metadata()`
    * [ ] Override `activation()`
    * [ ] Override `set_activation()`
    * [ ] Override `signature()`
    * [ ] Override `set_signature()`
    * [ ] Override `parent()`
    * [ ] Override `set_parent()`
    * [ ] Override `of_virtual_slot()`
    * [ ] Override `set_of_virtual_slot()`
    * [ ] Override `overriden_by()`
    * [ ] Override `overrides_method()`
    * [ ] Override `set_overrides_method()`
    * [ ] Override `is_final()`
    * [ ] Override `set_is_final()`
    * [ ] Override `is_static()`
    * [ ] Override `set_is_static()`
    * [ ] Override `is_abstract()`
    * [ ] Override `set_is_abstract()`
    * [ ] Override `is_external()`
    * [ ] Override `set_is_external()`
    * [ ] Override `is_overriding()`
    * [ ] Override `set_is_overriding()`
    * [ ] Override `is_async()`
    * [ ] Override `set_is_async()`
    * [ ] Override `is_generator()`
    * [ ] Override `set_is_generator()`
    * [ ] Override `is_constructor()`
    * [ ] Override `set_is_constructor()`
  * [ ] MethodSlotAfterSubstitution extends MethodSlot
    * [ ] Description: Represents a method slot after an indirect type substitution.
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