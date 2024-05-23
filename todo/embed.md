# Embed

## Embed meta-data applied to variable definition

* [ ] Handle SWF (default for .swf files)
* [ ] Handle image (default for .png, .jpg, .jpeg and .bmp files)
* [ ] Handle `application/octet-stream`
  * [ ] Handle `encrypted="true"`
* [ ] Handle font

## Embed meta-data applied to class definition

* [ ] Do not know what to handle; research it.

## Handling SWF

A class is generated that loads the SWF from a `ByteArray`:

```as3
package {
    import flash.compiler.embed.EmbeddedMovieClip;
   
    public class §Foo_swf$07cd5e28ac9a60a3fdf36db1994e52a6-435680683§ extends EmbeddedMovieClip {
        public function §Foo_swf$07cd5e28ac9a60a3fdf36db1994e52a6-435680683§() {
            super(new §Foo_swf$07cd5e28ac9a60a3fdf36db1994e52a6-435680683ByteArray§(), 500, 375);
        }
    }
}
```