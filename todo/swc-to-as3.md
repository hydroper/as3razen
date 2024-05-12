# SWC to AS3

Compiling external library SWC to AS3 would be ideal to have proper ASDoc and more use of type parameters from the AIR globals.

It is simple since method bodies do not have to be visited; just native code would be generated for external declarations inside `package` blocks.

## Namespaces

User namespaces are identified as `flash_proxy`, or `AS3`, and any unrecognized ones should cause an error, and definitions for them are implicitly generated in the resulting AS3 code.

## Imports

Packages are imported by wildcard based in the open namespace set. It should not cause any conflict for the AIR globals, so it is assumed to be safe to do this.