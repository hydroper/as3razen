# Specifying sources

## includeSources

The `includeSources` option of a FXRazen package includes sources recursively.

## excludeSources

The `excludeSources` option of a FXRazen package excludes sources recursively, useful for excluding `include "file.as";` directive's files.

## sourcePath

The `sourcePath` option, different from Flex, must be used alongside `includeSources` and is used as the base directory of the top-level package for MXML files. `sourcePath` is not an array unlike in Flex.

MXML files have their package determined based in the `sourcePath` directory, using the directory hierarchy.

Usually, `includeSources` and `sourcePath` may be set to point to the same directory.

For example:

```plain
src/
    com/
        foo/
            bar/
                CompA.mxml
```

Given `includeSources` is `["src"]` and `sourcePath` is `"src"`, the ActionScript 3 package of the `CompA.mxml` file is equivalent to `com.foo.bar`.