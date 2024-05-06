# Flex

## MXML

MXML components are required to contain a `fx:Package` element indicating the package they belong to. Use the empty string for the top-level package.

```xml
<?xml version="1.0"?>
<s:Application xmlns:fx="http://ns.hydroper.com/mxml/2024" xmlns:s="library://ns.hydroper.com/spark">
    <fx:Package>foo.bar</fx:Package>
</s:Application>
```