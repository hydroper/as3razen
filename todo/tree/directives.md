# Directives

## Phases

* [ ] Before a directive is verified, it is removed from the list of remaining directives to verify in the same block.
* [ ] When a directive throws a defer error, the directive is added to the list of remaining directives to verify in the same block.

## Variable definitions

* [ ] Assign ASDoc to topmost variable slot
* [ ] Assign `[Bindable]` semantics to topmost variable slots

## Class definitions

* [ ] Assign ASDoc
* [ ] Assign location
* [ ] Assign every `[Event]` semantics to the class
* [ ] Mark unused

## Function definitions

Function definitions should have careful plannings. It involves caching the activation, setting phases (similiarly to destructuring), and avoiding verifying things twice (the signature, that is). They should also be able to do inference in the signature's result type depending on the `inferTypes` option.

Never ever let getters and setters have the wrong signature assigned to them; if they are invalid, just use a default signature matching their requirements.