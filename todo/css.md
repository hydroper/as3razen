# CSS

## Inherited properties

UI components have to be implemented with CSS in mind; therefore, certain AS3 properties should be nullable so that they can be inherited from the parent component:

* `color`
* `fontFamily`
* `fontSize`
* `fontStyle`
* `fontWeight`

## Import stylesheet from AS3

Applying a stylesheet to an `UIComponent` should be possible through a meta-data. This requires the compiler to generate special code for the constructor code such that the stylesheet is instantiated and assigned to the component, and then used in rendering.

```as3
package com.example.controls {
    import razen.core.*;

    [CSS("./style.css")]
    public class MyControl extends UIComponent {
        //
    }
}
```