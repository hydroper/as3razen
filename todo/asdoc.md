# ASDoc

## Vector

* [ ] Automatically move the `__AS3__.vec.Vector` class to the top-level package and remove documentation for that package.

## Sections

Additional Markdown sections may be described in the Razen package's `asdoc.xml` file, used when rendering the ASDoc documentation for a Razen package.

```xml
<?xml version="1.0"?>
<asdoc>
    <!-- Affects all sections.  -->
    <basePath>docs</basePath>

    <includeFiles>
        <include>image.png</include>
    </includeFiles>

    <!-- Mixed in index.html with the ActionScript packages. -->
    <home>
        <title>My Package's Home</title>
        <path>home.md</path>
    </home>
    <sections>
        <!-- foo.html -->
        <section>
            <title>Foo</title>
            <path>foo.md</path>
            <!-- basePath: docs/foo -->
            <children>
                <!-- foo/qux.html -->
                <section>
                    <title>Qux</title>
                    <path>qux.md</path>
                </section>
            </children>
        </section>
    </sections>
</asdoc>
```