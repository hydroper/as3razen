# Directives

## Defer

* [ ] Statements are verified only after directives, in two different verification methods (one verification method for directives, and one pass verification method for statements). Block statements, with the right scopes, are entered recursively for directives.
* [ ] Directives are always have a cache to prevent re-verification using the node mapping of SemanticHost; it may just be an invalidation thingy when it does not matter, such as for an use namespace directive.
* [ ] When at least one directive throws a defer error, the entire verification should reoccur next time.
* [ ] Addition: the former explanations should be expanded such that deferred verification occurs in compilation unit level.

## Scopes

Set scopes carefully within directive sequences. Sometimes inherit and enter; sometimes just overwrite it.

## Across compilation units

Across all compilation units, directives should be verified first, from a package to a class or method, and from a class to a method or property. After all directives are solved in these ranges, statements may be verified in one pass.

## Directives versus statements

The `DirectiveSubverifier::verify_directive()` method will verify a directive, for certain directives and the block statement, their subdirectives until a limit (for example, from class goes until methods, and from a block statement goes until subdirectives).

* `DirectiveSubverifier::verify_directives` will verify a list of directives and, in case it found any deferred part, it returns `Err` (but all directives are guaranteed to be have been verified).

The `StatementSubverifier::verify_statement()` method will verify a statement or all substatements from a directive such as a class or function definition. It does not throw a defer error; anything that defers will result into a verify error.

* `StatementSubverifier::verify_statements()` will verify a list of statements using `StatementSubverifier::verify_statement()`.

## Variable definitions

Procedure:

* [ ] Alpha
  * [ ] Decide whether to hoist the variable or not (`blockScope` on?)
  * [ ] Check the `static` attribute to know where the output name goes exactly.
  * [ ] Determine the system namespace according to the attribute combination (`static` + `protected` = `static protected`).
  * [ ] If the parent is a fixture, do not allow destructuring, in which case the pattern shall be invalidated.
  * [ ] Call `DestructuringDeclarationSubverifier::verify_pattern(...)` without deferring (alpha phase)
  * [ ] If the first topmost variable binding's slot is not invalidated
    * [ ] Assign ASDoc to the first topmost variable binding's slot.
    * [ ] Assign meta-data to the first variable binding's slot.
* [ ] Beta
  * [ ] If a binding is a simple identifier
    * [ ] Try resolving type annotation if any; if resolved
      * [ ] If a binding's slot is not invalidated
        * [ ] Update the binding slot's static type
* [ ] Delta
  * [ ] If a binding is a simple identifier
    * [ ] If binding slot's not invalidated and its static type is unresolved
      * [ ] Try resolving type annotation if any; if resolved
        * [ ] Update the binding slot's static type
* [ ] Epsilon
  * [ ] Check if a property of the same name is defined in an inheritance parent. (Since destructuring is not allowed in fixtures, this only considers simple identifier patterns.)
    * [ ] Report a *redefining* error if a base property is found.
  * [ ] Handle the `[Bindable]` meta-data for simple identifier patterns
  * [ ] Handle the `[Embed]` meta-data for simple identifier patterns
* [ ] Omega
  * [ ] Let *init* be `None`.
  * [ ] Try resolving type annotation if any; defer if unresolved.
  * [ ] If there is an initialiser
    * [ ] If there is a type annotation
      * [ ] Implicitly coerce it to the annotated type and assign the result to *init*.
    * [ ] Else
      * [ ] Assign the result of verification of the initialiser into *init*.
  * [ ] Lazy initialise *init1* (`cached_var_init`)
    * [ ] If *init* is some and the compiler option `inferTypes` is on
      * [ ] Return *init*
    * [ ] Else
      * [ ] If there is no type annotation
        * [ ] Return a value of the `*` type.
      * [ ] Else
        * [ ] Return a value whose type is the annotated type.
  * [ ] Call `DestructuringDeclarationSubverifier::verify_pattern(...)?` using *init1*
  * [ ] Remove *init1* from `cached_var_init`
  * [ ] If there is no type annotation and (*init* is none or `inferTypes` is off)
    * [ ] Report a warning
  * [ ] If variable is marked constant, is not `[Embed]` and does not contain an initializer
    * [ ] Report an error.

## Inheritance

* [ ] For classes and interfaces, right after the phase in which the inheritance is solved, ensure the inheritance is not circular (an inherited type must not be equals to or a subtype of the inheritor type) by reporting a verify error in such case.
* [ ] For definitions within classes and interfaces, ensure they either override a method or do not redefine a previously defined property.

## Class initialiser method

Note that statements and static binding initializers within a class or enum block contribute code to the class initialiser method of AVM2, so control flow analysis should go from there rather than in the parent's initialiser (i.e. the package or top level).

## Class definitions

* [ ] Assign ASDoc
* [ ] Assign location
* [ ] Read the `[Options]` meta-data and apply `Options` classes restrictions
* [ ] Assign every `[Event]` semantics to the class
* [ ] Handle the `[Bindable]` meta-data right after variables are declared
* [ ] Handle the `[Embed]` meta-data.
* [ ] Assign attributes correctly (`static`, `dynamic`, `abstract`, and `final`)
* [ ] Mark unused

## Interface definitions

* [ ] Assign ASDoc
* [ ] Assign location
* [ ] Assign every `[Event]` semantics to the interface
* [ ] Mark unused
* [ ] For the interface block, verify only top-level function definitions

## Function definitions

Function definitions should have careful plannings. It involves caching the activation, setting phases (similiarly to destructuring), and avoiding verifying things twice (the signature, that is). They should also be able to do inference in the signature's result type depending on the `inferTypes` option.

Never ever let getters and setters have the wrong signature assigned to them; if they are invalid, just use a default signature matching their requirements.

Handle conflicting definitions properly, only moving forward in verification if the resulting slot is a method slot and not something else (it could be a variable slot or a class, for example).

## Getters/setters

Getters and setters have their own method of handling name conflict since they belong to a virtual slot.