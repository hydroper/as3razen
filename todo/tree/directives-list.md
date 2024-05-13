# Directives list

Tip: use a mapping from directive to phase for certain of the following directives. Clear that mapping on `reset_state()`.

* [ ] Variable definition
* [ ] Function definition
* [ ] Class definition
* [ ] Enum definition
* [ ] Interface definition
* [ ] Type definition
* [ ] Namespace definition
* [x] Block
* [x] Labeled statement
* [x] If statement
* [x] Switch statement
* [x] Switch type statement
* [x] Do statement
* [x] While statement
* [x] For statement
* [x] For..in statement
* [x] With statement
* [x] Try statement
* [x] Configuration directive
* [x] Import directive
* [ ] Use namespace directive
* [ ] Include directive
* [ ] Normal configuration directive
  * [ ] Evaluate constant with `ExpressionSubverifier::eval_config_constant()`
* [ ] Package concatenation directive
  * [ ] Do not allow adding circular packages (i.e. parent packages or the same package)
  * [ ] Some aliases should result in unresolved temporarily
* [ ] Directive injection