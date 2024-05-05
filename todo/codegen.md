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