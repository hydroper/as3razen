# Verifier

## External mode

In external mode, the verifier puts some restrictions, such as requiring only `native` methods and empty package and global initialization code. It is an alternative to including external SWCs, where all functions must be marked native for external purposes.

This will be common for the built-ins. For example, instead of airglobal.swc, it is desirable to have external sources with ASDoc comments.

Moreover, the external mode is up to the codegen to corelate a series of compilation units with.

## Locations

Do not forget to set source locations of entities such as classes and variables.

* [ ] Aliases
* [ ] Classes
* [ ] Enumerations
* [ ] Interfaces
* [ ] Type parameter types
* [ ] Variable slots
* [ ] Virtual slots
* [ ] Method slots

## ASDoc

* [ ] Set ASDoc comments properly, except for aliases.

## Meta-data

* [ ] Set meta-data properly, except for aliases.
* [ ] Handle Flex `[Bindable]`
* [ ] Handle Flex `[Embed]`
* [ ] Handle Flex `[Event]`

### @copy

* [ ] Correct anchor links from original source path to substitution source path.

### @inheritDoc

* [ ] Correct anchor links from original source path to substitution source path.

## Namespaces

* [ ] Throw a verify error if the namespace's local name conflicts with that of a configuration namespace in `host.config_constants()`.
* [ ] Combining the `static protected` modifiers in an annotatable directive indicates a `SystemNamespaceKind::StaticProtected` system namespace.
* [ ] Set ASDoc comments properly for explicit or user namespaces.
* [ ] `namespace ns1;` creates an `internal` system namespace belonging to a hardcoded package created in the fly, rather than an `UserNamespace`.
* [ ] `namespace ns1 = "...";` creates an user namespace (`UserNamespace`; not `ExplicitNamespace`).

## Packages

* [ ] Before analyzing a definitions in a set of programs, traverse the packages and the packages in all top-level `include` directives (as well as those in top-level block statements) to create them properly before ever hitting the `import ns.**;` and `public += ns.**;` directives.

* [ ] Besides the pre packages pass, another pass, specifically for the AS3 language built-ins (the top-level package and `__AS3__.vec` are taken into consideration, for efficiency), is ideal for optimization: "predefine" classes partially and cache them at the class definition, which is important for primitive types used in name lookups (such as `String` and `Number`).

* [ ] The topmost scope of a package is an activation with `set_is_package_initialization(true)`, from which the package scope is subsequent.

## Import

### Name import

* [ ] A `import x = ns.y;` directive assigns an alias `x` to the enclosing scope.

* [ ] An `import ns.y;` directive contributes a `PackagePropertyImport` to the enclosing scope, which may be fully qualified (`ns.y`) or lexically referred to (`y`).

### Wildcard import

* [ ] An wildcard import contributes a `PackageWildcardImport` to the enclosing scope. No need to contribute an open namespace, since the lookup in wildcard import happens with the open namespace set and any `public` namespace.

* [ ] An `import foons = foo.*;` assigns an alias to a `PackageWildcardImport` to the enclosing package.

### Recursive import

* [ ] An `import ns.**;` contributes a `PackageRecursiveImport` to the enclosing scope.

* [ ] An `import foons = foo.**;` assigns an alias to a `PackageRecursiveImport` to the enclosing package.

## Package concatenation

The following:

```
package foo.bar {
    public += qux.**;
}
```

contributes multiple concatenated packages from `qux.**` to the `foo.bar.*` package.

## Package shadowing

* [x] Packages shadow variable names through fully qualified name comparisons against `Identifier` followed by zero or more `"." IdentifierName` sequences in each property operation.

## Activations

* [x] Detect captured properties by calling `set_property_has_capture()` in the parent activation when resolving a lexical reference and the activation to which it belongs, if any, is different from the current activation.
* [ ] Set `this()` properly in activations. For class static methods, global initialization code, and package initialization code, `this()` should always be `None`.

## Global initialization code

* [ ] For global initialization code, the topmost activation must set `public_ns()` and `internal_ns()`, for use with reserved namespace expressions and attribute combinations.

## Methods

* [ ] Set `is_async()`, `is_generator()`, and `is_constructor()` properly in method slots.
* [ ] Auto wrap asynchronous method's result type from signature into `Promise` if not already a `Promise`.

## Parameterized types

* Parameterized types, when referred to without an immediately following `.<...>` sequence, are equivalent to `T.<*, ...>`.
  * [x] Lexical reference
  * [x] Dot operator

## Array type

* [ ] In ABC, a type parameter is automatically added for `Array` and `__AS3__.vec.Vector`.
* [ ] In source, a type parameter is automatically added for `Array` and `__AS3__.vec.Vector`.

## Property access

* [ ] For dot and brackets operators, after filtering for shadowing package names
  * At first check if the base is a reference to a type (`PackageReferenceValue` or `ScopeReferenceValue` with a `property` that matches a type)
    * Lookup for property in that type first
  * Finally, if the first check returned `Ok(None)` or did not occur, lookup for property in the reference's resulting data type (e.g. `Class`).

## Open namespaces

Open namespaces properly everywhere.

* [ ] Package definitions opens the package's `internal`
* [ ] Class definition opens its `private`, `protected`, `static protected`, and also the inherited classes's `protected` and `static protected`.
* [ ] Enum definition opens its `private`.

## Parents

* [ ] Set parents correctly in all definitions.
  * [ ] Enclosing scope, type, or package, for example.

## Attributes

* [ ] Restrict definitions at package block to be either `public` or `internal`.
* [ ] Restrict definitions at top-level to be `internal`.
* [ ] Definitions at the top-level of a class may be in any namespace.
* [ ] Restrict user-defined namespaces to be used only at the top-level of class definitions.

## Getters and setters

* [ ] Invoke `set_of_virtual_slot()` properly.

## Inline constants

* [x] Expand inline constants

## Enums

* [ ] Define member slots with a `T!` non-null data type instead of `T` as-is.
* [ ] Perform mapping from member String to Number and from String to member variable slot

## Activations

* [ ] In most `FunctionCommon`, `this` is set to always be of the `*` data type.

## Signatures

* [ ] Restrict the rest parameter to be `Array.<T>`. If it is untyped, it defaults to `[*]`.
* [ ] Restrict the rest parameter's data type to not be a non-nullable layer over `Array.<T>`.

## Initial scope

* [ ] The initial scope of a package opens the `AS3` namespace (when at AS3 mode) and imports the top-level package.
* [ ] The initial scope of a program's directive sequence opens the `AS3` namespace (when at AS3 mode) and imports the top-level package.

## Options classes

* [ ] Mark them implicitly final.
* [ ] Restrict them to extend only Object.
* [ ] Restrain all fields to be writable.

## Constructors

* [ ] Require a non default constructor (a constructor with a non-empty parameter list) to be invoked from descending constructors.

## Hoisting

* [ ] If block-scoping is on, variables do not hoist to the activation (but functions still do).

## Return statement

* [ ] Report the following errors for the `return` statement used in global or package initialization code. To detect this:
  * [ ] Make sure to place either `set_is_global_initialization(true)` or `set_is_package_initialization(true)` in the activation.

```plain
Error: The return statement cannot be used in global initialization code.
Error: The return statement cannot be used in package initialization code.
```

## Top level

* [ ] The topmost scope of top level code is an activation with `set_is_global_initialization(true)`.

## Classes and enumerations

* [ ] Do not allow destructuring in variable bindings belonging to class or enum top level declarations.