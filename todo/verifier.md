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

Before analyzing a definitions in a set of programs, traverse the packages and the packages in all top-level `include` directives (as well as those in top-level block statements) to create them properly before ever hitting the `import ns.**;` and `public += ns.**;` directives.

## Import

### Recursive import

The following:

```
import ns.**;
```

will contribute multiple `PackageWildcardImport`s to the enclosing scope and open all `public` namespaces of the respective packages.

## Package shadowing

* [ ] Packages shadow variable names through fully qualified name comparisons against `Identifier` followed by zero or more `"." IdentifierName` sequences in each property operation.
