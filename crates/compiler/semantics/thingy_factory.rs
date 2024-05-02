use crate::ns::*;

pub struct ThingyFactory<'a>(pub(crate) &'a SemanticHost);

impl<'a> ThingyFactory<'a> {
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
        if let Some(qn_mappings) = ns_mappings.get_mut(&ByAddress(namespace.clone())) {
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
        ns_mappings.insert(ByAddress(namespace.clone()), qn_mappings);
        qn
    }
}