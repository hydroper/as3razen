# Codegen

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

## Package initialization

* [ ] Omit package initialization code if it is empty.

## Global initialization

* [ ] Omit global initialization code if it is empty.