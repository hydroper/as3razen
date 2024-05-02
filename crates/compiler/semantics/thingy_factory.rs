use crate::ns::*;

pub struct ThingyFactory<'a>(pub(crate) &'a SemanticHost);

impl<'a> ThingyFactory<'a> {
    pub fn create_public_namespace(&self, parent: Option<Thingy>) -> Namespace {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Public, parent).into()
    }

    pub fn create_private_namespace(&self, parent: Option<Thingy>) -> Namespace {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Private, parent).into()
    }

    pub fn create_protected_namespace(&self, parent: Option<Thingy>) -> Namespace {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Protected, parent).into()
    }

    pub fn create_internal_namespace(&self, parent: Option<Thingy>) -> Namespace {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Internal, parent).into()
    }

    pub fn create_explicit_namespace(&self, uri: String) -> Namespace {
        let mut mappings = self.0.explicit_namespaces.borrow_mut();
        if let Some(ns) = mappings.get(&uri) {
            return ns.clone();
        }
        let ns: Namespace = ExplicitNamespace::new(&self.0.arena, uri.clone()).into();
        mappings.insert(uri, ns.clone());
        ns
    }

    pub fn create_user_namespace(&self, uri: String) -> Namespace {
        let mut mappings = self.0.user_namespaces.borrow_mut();
        if let Some(ns) = mappings.get(&uri) {
            return ns.clone();
        }
        let ns: Namespace = UserNamespace::new(&self.0.arena, uri.clone()).into();
        mappings.insert(uri, ns.clone());
        ns
    }

    pub fn create_qname(&self, namespace: &Namespace, local_name: String) -> QName {
        let mut ns_mappings = self.0.qnames.borrow_mut();
        if let Some(qn_mappings) = ns_mappings.get_mut(namespace) {
            if let Some(qn) = qn_mappings.get(&local_name) {
                return qn.clone();
            }
            let qn = QName(Rc::new(QName1 {
                m_namespace: namespace.clone(),
                m_local_name: local_name.clone(),
            }));
            qn_mappings.insert(local_name, qn.clone());
            return qn;
        }
        let qn = QName(Rc::new(QName1 {
            m_namespace: namespace.clone(),
            m_local_name: local_name.clone(),
        }));
        let mut qn_mappings = HashMap::new();
        qn_mappings.insert(local_name, qn.clone());
        ns_mappings.insert(namespace.clone(), qn_mappings);
        qn
    }

    pub fn create_namespace_set(&self, list: SharedArray<Namespace>) -> NamespaceSet {
        // Do not intern namespace sets for now.
        NamespaceSet::new(&self.0.arena, list)
    }

    /// Interns a package from a fully qualified name.
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(host.factory().create_package(["foo", "bar"]).fully_qualified_name(), "foo.bar");
    /// ```
    pub fn create_package<'b>(&self, name: impl IntoIterator<Item = &'b str>) -> Package {
        self.create_package_1(&name.into_iter().collect())
    }

    fn create_package_1(&self, name: &Vec<&str>) -> Package {
        let mut result: Package = self.0.top_level_package.clone();
        for name_1 in name {
            let name_1 = (*name_1).to_owned();
            let result_1 = result.subpackages().get(&name_1);
            if let Some(result_1) = result_1 {
                result = result_1;
            } else {
                let result_1 = Package::new(&self.0.arena, name_1.clone());
                result_1.set_parent(Some(result.clone().into()));

                // Assign namespaces
                result_1.set_public_ns(Some(self.create_public_namespace(Some(result_1.clone().into()))));
                result_1.set_internal_ns(Some(self.create_internal_namespace(Some(result_1.clone().into()))));

                result.subpackages().set(name_1, result_1.clone());
                result = result_1;
            }
        }
        result
    }
}