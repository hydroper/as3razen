use crate::ns::*;

pub struct Unused<'a>(pub &'a SemanticHost);

impl<'a> Unused<'a> {
    pub(crate) fn all(&self) -> std::cell::Ref<Vec<Thingy>> {
        self.0.unused_things()
    }

    pub(crate) fn add(&self, thing: &Thingy) {
        self.0.add_unused_thing(thing);
    }

    pub fn mark_used(&self, property: &Thingy) {
        if property.is::<InvalidationThingy>() {
            return;
        }
        let qn = property.name();
        if !qn.in_public_or_protected_ns() {
            self.0.remove_unused_thing(property);
        }
    }
}