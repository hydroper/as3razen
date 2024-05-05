# Specifying sources

## includeSources

The `includeSources` option of a FXRazen package includes sources recursively.

## excludeSources

The `excludeSources` option of a FXRazen package excludes sources recursively, useful for excluding `include "file.as"` directive's files.

## fx:Package

Because there is no notion of source paths in the compiler unlike in Apache Flex, users are required to enter the `<fx:Package>` tag in their MXML components to indicate a package.

```xml
<?xml version="1.0"?>
<b:Application xmlns:fx="http://ns.hydroper.com/mxml/2024" xmlns:b="library://ns.hydroper.com/betax">
    <fx:Package>foo.bar</fx:Package>
</b:Application>
```