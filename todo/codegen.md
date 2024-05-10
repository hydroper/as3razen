# Codegen

## External projects

External FXRazen projects are external libraries that do not generate any code, typically used for builtins such as AIR globals. In these cases, only the verifier is responsible for handling the semantic model.

## Vector Data Type

* [ ] Generate a multiname for a `Vector` type with type arguments as follows:

```plain
TypeName(QName(PackageNamespace("__AS3__.vec"),"Vector")<QName(PackageNamespace(""),"Number")>)
```

* [ ] Generate a `Class` object for a `Vector` type with type arguments as follows:

```plain
getlex QName(PackageNamespace("__AS3__.vec"),"Vector")
getlex QName(PackageNamespace(""),"Number")
applytype (1)
```

## Bindable

See the [To Do List](flex.md) for Flex for the `[Bindable]` meta-data.

* [ ] Implement `[Bindable(...)]` at class definitions
* [ ] Implement `[Bindable(...)]` at variable definitions
* [ ] Implement `[Bindable(...)]` at setter definitions

## Embed

No notes as of yet.

## Conversions

* [ ] Visit conversion values in the node mapping carefully and travel until the topmost value of the conversion and pass it as a parameter to the node visitor rather than just directly taking the semantic thing from the node's mapping.

## Constant values

* [ ] Visit constant values in the node mapping before generating code for an expression. Constant values should yield a cheap AVM2 constant value.

## Package initialization

* [ ] Omit package initialization code if it is empty.

## Global initialization

* [ ] Omit global initialization code if it is empty.

## Call operator

* [ ] Calling `Array(...)` behaves the same as `new Array(...)`.

## Prototype

* [ ] Do not contribute the "prototype" property from a class object to the AVM2 bytecode. It should be defined implicitly by AVM2.

## Non-null assertion operator

* [ ] For the `o!` operation, do not generate any assertion code, for efficiency (just used for type checking really).