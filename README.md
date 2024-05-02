# AS3Razen

AS3Razen is an ActionScript 3 compiler in alpha development phase.

AS3Razen aims to comply with either the Apache Flex or Royale compilers which are implemented in the Java language using the Maven ecosystem. AS3Razen would be completely in Rust, using the Cargo (crates.io) ecosystem of dependencies.

## Motivation

I enjoy developing compilers from scratch.

I have found ASDoc's implementation to be problematic in certain ways, and there is no compiler option in Flex to allow optional Markdown support.

I would like to have a handy package management in AS3 comparable to that of Cargo for Rust.

## Research

The following links may be useful:

[Rust for ActionScript developers](https://github.com/hydroper/as3parser/wiki/Rust-for-ActionScript-developers)

[Building a compiler](https://github.com/hydroper/as3parser/blob/master/docs/building-a-compiler.md)

[Semantic model](https://github.com/hydroper/as3parser/wiki/Semantic-model)

[Reference documents](https://github.com/hydroper/as3parser/blob/master/docs/references.md)

## Old code

I was for a short time working in a dialect codenamed Jet, which I've abandoned. It contains code that may be mimmicked in semantic cases, such as type substitution, and a few logical parts.

[Jet compiler](https://github.com/hydroper-jet/private-compiler/tree/master/src/compiler)

## License

Apache 2.0, copyright © Hydroper