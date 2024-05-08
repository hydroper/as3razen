# SWC to AS3

Compiling external library SWC to AS3 would be ideal to have proper ASDoc and more use of type parameters from the AIR globals.

It is simple since method bodies do not have to be visited; just native code would be generated for external declarations inside `package` blocks.

## Namespaces

Namespaces are assumed to be aliased somewhere in a namespace slot trait, so that they are used as control access modifiers in annotatable directives, otherwise an error should be reported.