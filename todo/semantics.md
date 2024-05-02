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
  * [x] Alias
  * [ ] Types
    * [x] Any type
    * [x] Void type
    * [x] Class type
    * [ ] Enum type
      * [ ] Override `name()`
      * [ ] Override `parent()`
      * [ ] Override `set_parent()`
      * [ ] Override `private_ns()`
      * [ ] Override `set_private_ns()`
      * [ ] Override `enum_members()`
      * [ ] Override `is_abstract()`
        * Return false
      * [ ] Override `is_dynamic()`
        * Return false
      * [ ] Override `is_final()`
        * Return true
      * [ ] Override `is_option_set()`
        * Return false
      * [ ] Override `extends_class()`
        * Return `Object`
      * [ ] Override `properties()`
      * [ ] Override `prototype()`
      * [ ] Override `includes_null()`
      * [ ] Override `includes_undefined()`
      * [ ] Override `asdoc()`
      * [ ] Override `set_asdoc()`
      * [ ] Override `metadata()`
      * [ ] Override `to_string_1()`
    * [ ] Interface type
    * [ ] Type after substitution
      * [ ] Override fully qualified name
    * [ ] Tuple type
    * [ ] Function type
    * [ ] Nullable type
    * [ ] Non nullable type
    * [ ] Type parameter type
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