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

| Title | Description |
| ----- | ----------- |
| [Rust for ActionScript developers](https://github.com/hydroper/as3parser/wiki/Rust-for-ActionScript-developers) | A Rust quickstart introduction. |
| [Building a compiler](https://github.com/hydroper/as3parser/blob/master/docs/building-a-compiler.md) | Several important points about an ActionScript 3 compiler. |
| [Semantic model](https://github.com/hydroper/as3parser/wiki/Semantic-model) | Techniques for the semantic model. Pretty simple actually. |
| [Reference documents](https://github.com/hydroper/as3parser/blob/master/docs/references.md) | Links to documents such as language specifications, AVM2 overview, SWF 19 spec., and more. |
| [Jet code](https://github.com/hydroper-jet/privcompiler/tree/master/src/compiler) | I was for a short time working in a dialect codenamed Jet, which I've abandoned. It contains code that may be mimmicked in semantic cases, such as type substitution, and a few logical parts. |

## To do

| Title | Description |
| ----- | ----------- |
| [APM](apm.md) | |
| [Compilation units](compilation-units.md) | |
| [Misc.](misc.md) | |
| [Semantics](semantics.md) | |
| [Unused warning](unused-warning.md) | |
| [Verifier](verifier.md) | |

## License

Apache 2.0, copyright © Hydroper