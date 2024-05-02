# FXRazen

FXRazen is an ActionScript 3 compiler in alpha development phase, for use with the AIR technology developed by Samsung HARMAN.

FXRazen aims to comply with either the Apache Flex or Royale compilers which are implemented in the Java language using the Maven ecosystem. FXRazen would be completely in Rust, using the Cargo (crates.io) ecosystem of dependencies.

## Motivation

**Goal**

I enjoy developing a compiler from scratch and have spent too much time implementing parsers, and a few time spans with semantic analysis. I am by no means expert with semantic analysis, but I have learned certain lessons for preventing bugs such as those in type substitution through the use of throwing *defer* verification errors instead of using a number of phases for partially verifying a series of compilation units.

**ASDoc**

I have found ASDoc's implementation to be problematic in certain ways, and there is no compiler option in Flex to allow optional Markdown support.

**Package manager**

I would like to have friendly package management in ActionScript 3 comparable to that of Cargo for Rust. Distriqt's APM is interesting in that it handles ANEs, which I should watch for later when implementing a package manager. I just think though that the package manager should be integrated with the language's compiler for more flexibility.

## Research

The following links may be useful:

[Rust for ActionScript developers](https://github.com/hydroper/as3parser/wiki/Rust-for-ActionScript-developers)

[Building a compiler](https://github.com/hydroper/as3parser/blob/master/docs/building-a-compiler.md)

[Semantic model](https://github.com/hydroper/as3parser/wiki/Semantic-model)

[Reference documents](https://github.com/hydroper/as3parser/blob/master/docs/references.md)

### Old code

I was for a short time working in a dialect codenamed Jet, which I've abandoned. It contains code that may be mimmicked in semantic cases, such as type substitution, and a few logical parts.

[Jet compiler](https://github.com/hydroper-jet/privcompiler/tree/master/src/compiler)

## License

Apache 2.0, copyright © Hydroper