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
      * [x] Override `name()`
      * [x] Override `parent()`
      * [x] Override `set_parent()`
      * [x] Override `private_ns()`
      * [x] Override `set_private_ns()`
      * [x] Override `protected_ns()`
      * [x] Override `set_protected_ns()`
      * [x] Override `static_protected_ns()`
      * [x] Override `set_static_protected_ns()`
      * [x] Flex `[Event]` mapping
      * [x] Override `is_abstract()`
      * [x] Override `set_is_abstract()`
      * [x] Override `is_final()`
      * [x] Override `set_is_final()`
      * [x] Override `is_dynamic()`
      * [x] Override `set_is_dynamic()`
      * [x] Override `is_option_set()`
      * [x] Override `set_is_option_set()`
      * [x] Override `constructor_method()`
      * [x] Override `set_constructor_method()`
      * [x] Override `implements()`
      * [x] Override `extends_class()`
      * [x] Override `set_extends_class()`
      * [x] Override `type_parameters()`
      * [x] Override `set_type_parameters()`
      * [x] Override `properties()`
      * [x] Override `prototype()`
      * [x] Override `known_subclasses()`
      * [x] Override `includes_null()`
      * [x] Override `includes_undefined()`
      * [x] Override `asdoc()`
      * [x] Override `set_asdoc()`
      * [x] Override `metadata()`
      * [x] Override `to_string_1()`
    * [ ] Enum type
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