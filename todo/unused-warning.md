# Unused warning

* [ ] Wildcard `import`s must be tracked into an unique collection where they are considered "unused".
  * [ ] Remove the wildcard `import` from that collection once a name is read from it.
* [ ] Non `public` entities (such as aliases, classes, and variable slots) all must all be tracked into an unique collection where they are considered "unused".
  * [ ] Remember of `import` aliases
  * [ ] Remove an entity from that unique collection when it is read.
* [ ] Entities resulting after type substitution, when read, should have their original entity removed from that unique collection.