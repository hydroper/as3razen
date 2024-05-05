# Verifier

## External mode

In external mode, the verifier should not contribute any bytecode and just semantic model, marking everything as "external". It is an alternative to including external SWCs, where all functions must be marked native for external purposes.

This will be common for the built-ins. For example, instead of airglobal.swc, it is desirable to have external sources with ASDoc comments.

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

* [ ] Set meta-data proeprly, except for aliases.

### @copy

* [ ] Correct anchor links from original source path to substitution source path.

### @inheritDoc

* [ ] Correct anchor links from original source path to substitution source path.

## Namespaces

* [ ] Combining the `static protected` modifiers in an annotatable directive indicates a `SystemNamespaceKind::StaticProtected` system namespace.
* [ ] Set ASDoc comments properly for explicit or user namespaces.
* [ ] `namespace ns1;` creates an `internal` system namespace, rather than an explicit or user namespace.

## Packages

* [ ] Before analyzing a definitions in a set of programs, traverse the packages and the packages in all top-level `include` directives (as well as those in top-level block statements) to create them properly before ever hitting the `import ns.**;` and `public += ns.**;` directives.

* [ ] Besides the pre packages pass, another pass, specifically for the AS3 language built-ins (the top-level package and `__AS3__.vec` are taken into consideration, for efficiency), is ideal for optimization: "predefine" classes partially and cache them at the class definition, which is important for primitive types used in name lookups (such as `String` and `Number`).

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

* [ ] Packages shadow variable names through fully qualified name comparisons against `Identifier` followed by zero or more `"." IdentifierName` sequences in each property operation.

## Activations

* [ ] Detect captured properties by calling `set_property_has_capture()` in the parent activation when resolving a lexical reference and the activation to which it belongs, if any, is different from the current activation.

## Parameterized types

* [ ] Parameterized types, when referred to without an immediately following `.<...>` sequence, are equivalent to `T.<*>`.

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