use crate::ns::*;

pub struct PropertyLookup<'a>(pub &'a SemanticHost);

#[derive(Clone)]
pub enum PropertyLookupKey {
    String(String),
    /// Computed key.
    Value(Thingy),
}

impl PropertyLookupKey {
    pub fn thingy(&self, host: &SemanticHost) -> Result<Thingy, DeferError> {
        match self {
            Self::String(s) => {
                let string_type = host.string_type().defer()?;
                Ok(host.factory().create_string_constant(s.clone(), &string_type))
            },
            Self::Value(s) => Ok(s.clone()),
        }
    }

    pub fn static_type(&self, host: &SemanticHost) -> Result<Thingy, DeferError> {
        match self {
            Self::String(_) => host.string_type().defer(),
            Self::Value(s) => s.static_type(host).defer(),
        }
    }

    pub fn string_value(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            /*
            Self::Value(s) => {
                if s.is::<StringConstant>() {
                    Some(s.string_value())
                } else {
                    None
                }
            },
            */
            _ => None,
        }
    }

    pub fn double_value(&self, host: &SemanticHost) -> Result<Option<f64>, DeferError> {
        Ok(match self {
            Self::Value(d) => {
                if d.is::<NumberConstant>() {
                    let v = d.number_value();
                    Some(match &v {
                        NumberVariant::Number(d) => *d,
                        NumberVariant::Float(_) |
                        NumberVariant::Uint(_) |
                        NumberVariant::Int(_) => v.convert_type(&host.number_type().defer()?, host)?.as_double().unwrap(),
                    })
                } else {
                    None
                }
            },
            _ => None,
        })
    }
}

fn defer(thingy: &Thingy) -> Result<Thingy, PropertyLookupError> {
    if thingy.is::<UnresolvedThingy>() {
        Err(PropertyLookupError::Defer)
    } else {
        Ok(thingy.clone())
    }
}

fn map_defer_error<T>(result: Result<T, DeferError>) -> Result<T, PropertyLookupError> {
    result.map_err(|_| PropertyLookupError::Defer)
}

impl<'a> PropertyLookup<'a> {
    pub fn lookup_in_object(&self, base: &Thingy, open_ns_set: &SharedArray<Thingy>, qual: Option<Thingy>, key: &PropertyLookupKey) -> Result<Option<Thingy>, PropertyLookupError> {
        // If base is a value whose type is one of { XML, XMLList }, return a XML reference value.
        if base.is::<Value>() && [defer(&self.0.xml_type())?, defer(&self.0.xml_list_type())?].contains(&defer(&base.static_type(self.0))?) {
            let k = map_defer_error(key.thingy(self.0))?;
            return Ok(Some(self.0.factory().create_xml_reference_value(base, qual, &k)));
        }

        let string_key = key.string_value();
        let double_key = map_defer_error(key.double_value(self.0))?;

        // If base is a value whose type is one of { *, Object, Object! }, or
        // if key is not a String or Number constant,
        // return a dynamic reference value.
        if base.is::<Value>() && [self.0.any_type(), defer(&self.0.object_type())?].contains(&defer(&base.static_type(self.0))?.escape_of_non_nullable()) {
            let k = map_defer_error(key.thingy(self.0))?;
            return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
        }

        // If base is a class or enum
        if base.is_class_type_possibly_after_sub() || base.is::<EnumType>() {
            // Key must be a String constant
            let Some(local_name) = string_key else {
                return Ok(None);
            };

            // Qualifier must be a compile-time Namespace, otherwise return static dynamic reference.
            if qual.as_ref().map(|q| q.is::<Namespace>()).unwrap_or(true) {
                let k = map_defer_error(PropertyLookupKey::String(local_name).thingy(self.0))?;
                return Ok(Some(self.0.factory().create_static_dynamic_reference_value(base, qual, &k)));
            }

            for class in base.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
                // Defer if unresolved
                defer(&class)?;

                let r = self.get_qname(&class.properties(self.0), open_ns_set, qual.clone(), &local_name)?;
                if let Some(r) = r {
                    // Defer if unresolved
                    defer(&r.static_type(self.0))?;

                    return Ok(Some(map_defer_error(self.0.factory().create_static_reference_value(&class, &r))?));
                }
            }

            return Ok(None);
        }

        // If base is an interface
        if base.is_interface_type_possibly_after_sub() {
            // Key must be a String constant
            let Some(key) = string_key else {
                return Ok(None);
            };

            // Qualifier must be a compile-time Namespace, otherwise return static dynamic reference.
            if qual.map(|q| q.is::<Namespace>()).unwrap_or(true) {
                let k = map_defer_error(PropertyLookupKey::String(key).thingy(self.0))?;
                return Ok(Some(self.0.factory().create_static_dynamic_reference_value(base, qual, &k)));
            }

            return Ok(None);
        }

        // Base a little bit in https://github.com/hydroper-jet/privcompiler/blob/master/src/compiler/semantics/property_resolution.rs#L128
        // but read the semantics in the To Do list.

        todo()
    }

    pub fn lookup_in_scope_chain(&self, scope: &Thingy, qual: Option<Thingy>, key: &PropertyLookupKey) -> Result<Option<Thingy>, PropertyLookupError> {
    }

    /// Qualifier is assumed to be a compile-time Namespace.
    fn get_qname(&self, mapping: &NameMap, open_ns_set: &SharedArray<Thingy>, qual: Option<Thingy>, local_name: &str) -> Result<Option<Thingy>, PropertyLookupError> {
        if let Some(qual) = qual {
            Ok(mapping.get(&self.0.factory().create_qname(&qual, local_name.to_owned())))
        } else {
            mapping.get_in_ns_set(open_ns_set, local_name).map_err(|e| PropertyLookupError::AmbiguousReference(e.0))
        }
    }
}