# Conditional compilation

## Defining constants

The user should be able to define `NS::X` constants assigned to character data both in project configuration and command line, where the command line overrides the project configuration's constants.

Like in Flex, these constants are assigned to character data and, when used as inline constants, are evaluated as expressions in one verifier pass.

## Semantic host

* [ ] Defined constants are assigned to the `SemanticHost`'s `config_constants()` mapping.
* [ ] Cleanup: For every project, clear the previously defined constants and pass again the project and command line's constants.