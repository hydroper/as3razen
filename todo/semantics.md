# Semantic model

* [x] QName
* [x] NumberVariant
* [ ] Semantic host
* [x] NameMap
* [ ] Warning: Entities, including classes, aliases, variable slots and so on, all must have an optional "location" field pointing to where they were defined in the source code.
  * [ ] Alias
  * [ ] Class
  * [ ] Enum
  * [ ] Interface
  * [ ] Type parameter type
  * [ ] Variable slot
  * [ ] Virtual slot
  * [ ] Method slot
* [ ] Things
  * [x] Namespaces
  * [x] NamespaceSet
  * [x] Packages
  * [ ] PackageSet
    * [ ] Description: Package sets may be used in a future version for recursive package aliases, and possibly `q.**` XML namespaces in Flex rather than just `q.*`.
  * [ ] Alias
  * [ ] Types
    * [ ] Any type
    * [ ] Void type
    * [ ] Class type
    * [ ] Enum type
    * [ ] Interface type
    * [ ] Type after substitution
      * [ ] Override fully qualified name
    * [ ] Tuple type
    * [ ] Function type
    * [ ] Non nullable type
    * [ ] Type parameter type
  * [ ] Scopes
    * [ ] Field: Parent
    * [ ] Field: Open namespace set
    * [ ] Field: Package property imports
      * [ ] Description: Mapping from package to property
    * [ ] Field: Package wildcard imports
      * [ ] Description: Set of packages
    * [ ] WithScope
    * [ ] FilterOperatorScope
    * [ ] ActivationScope
    * [ ] ClassScope
    * [ ] EnumScope
    * [ ] InterfaceScope
    * [ ] PackageScope
  * [ ] Variable slot
  * [ ] Variable slot after substitution
    * [ ] Description: Represents a variable slot after an indirect type substitution.
  * [ ] Virtual slot
  * [ ] Virtual slot after substitution
    * [ ] Description: Represents a virtual slot after an indirect type substitution.
  * [ ] Method slot
  * [ ] Method slot after substitution
    * [ ] Description: Represents a method slot after an indirect type substitution.
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
* [ ] Type substitution