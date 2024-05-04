# Flex

## MXML

MXML components are required to contain a `fx:Package` element indicating the package they belong to. Use the empty string for the top-level package.

```xml
<?xml version="1.0"?>
<mw:Application xmlns:fx="http://ns.adobe.com/mxml/2009" xmlns:mw="library://ns.hydroper.com/masterwidget">
    <fx:Package>com.foo.bar</fx:Package>
</mw:Application>
```