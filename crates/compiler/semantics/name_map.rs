use crate::ns::*;

/// Represents a mapping from a qualified name to a thingy
/// (alias, variable slot, class, or any other thing).
/// 
/// `NameMap` is a mutable mapping managed by reference counting.
///
/// # Cloning
/// 
/// The `clone()` method of `NameMap` clones the mapping
/// by reference, and not by content.
/// 
/// Use `clone_content()` for cloning by content.
/// 
/// # Iteration
///
/// Iteration over the entries in the name mapping is
/// performed through the `borrow()` method.
///
/// ```ignore
/// for (qname, thingy) in name_map.borrow().iter() {
///     // Action
/// });
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NameMap(SharedMap<QName, Thingy>);

impl NameMap {
    pub fn new() -> Self {
        Self(SharedMap::new())
    }

    pub fn get(&self, name: &QName) -> Option<Thingy> {
        self.0.get(name)
    }

    /// Retrieves a thingy matching a local name in a namespace set.
    pub fn get_in_ns_set(&self, ns_set: &NamespaceSet, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            let ns1 = qname.namespace();
            let mut found_ns = false;
            for ns2 in ns_set.ns_set_list().iter() {
                if ns1 == ns2 {
                    found_ns = true;
                    break;
                }
            }
            if !found_ns {
                continue;
            }
            if qname.local_name() == local_name {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in any namespace.
    pub fn get_in_any_ns(&self, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            if qname.local_name() == local_name {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    pub fn set(&mut self, name: QName, thing: Thingy) {
        self.0.set(name, thing);
    }

    pub fn delete(&mut self, name: &QName) -> bool {
        self.0.remove(name).is_some()
    }

    /// The `borrow()` method may be used for iteration. Refer to the
    /// [`NameMap`] documentation for an example.
    pub fn borrow(&self) -> std::cell::Ref<HashMap<QName, Thingy>> {
        self.0.borrow()
    }

    /// Clones this `NameMap` by content, and not by reference.
    pub fn clone_content(&self) -> Self {
        Self(self.0.clone_content())
    }
}