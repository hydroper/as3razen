# Verifier

## Locations

Do not forget to set source locations of entities such as classes and variables.

* [ ] Aliases
* [ ] Classes
* [ ] Enumerations
* [ ] Interfaces
* [ ] Variable slots
* [ ] Virtual slots
* [ ] Method slots

## ASDoc

* [ ] Set ASDoc comments properly.

### @copy

* [ ] Correct anchor links from original source path to substitution source path.

### @inheritDoc

* [ ] Correct anchor links from original source path to substitution source path.

## Namespaces

* [ ] Combining the `static protected` modifiers in an annotatable directive indicates a `SystemNamespaceKind::StaticProtected` system namespace.
* [ ] Set ASDoc comments properly for explicit or user namespaces.
* [ ] `namespace ns1;` creates an `internal` system namespace, rather than an explicit or user namespace.

## Package shadowing

* [ ] Packages shadow variable names through fully qualified name comparisons against `Identifier` followed by zero or more `"." IdentifierName` sequences in each property operation.