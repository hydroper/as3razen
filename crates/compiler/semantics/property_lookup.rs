use crate::ns::*;

pub struct PropertyLookup<'a>(pub &'a SemanticHost);

#[derive(Clone)]
pub enum PropertyLookupKey {
    LocalName(String),
    Computed(Thingy),
}

impl PropertyLookupKey {
    pub fn computed_or_local_name(&self, host: &SemanticHost) -> Result<Thingy, DeferError> {
        match self {
            Self::LocalName(s) => {
                let string_type = host.string_type().defer()?;
                Ok(host.factory().create_string_constant(s.clone(), &string_type))
            },
            Self::Computed(s) => Ok(s.clone()),
        }
    }

    pub fn static_type(&self, host: &SemanticHost) -> Result<Thingy, DeferError> {
        match self {
            Self::LocalName(_) => host.string_type().defer(),
            Self::Computed(s) => s.static_type(host).defer(),
        }
    }

    pub fn local_name(&self) -> Option<String> {
        match self {
            Self::LocalName(s) => Some(s.clone()),
            /*
            Self::Computed(s) => {
                if s.is::<StringConstant>() {
                    Some(s.local_name())
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
            Self::Computed(d) => {
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
        let local_name = key.local_name();
        let double_key = map_defer_error(key.double_value(self.0))?;

        // If base is a class or enum
        if base.is_class_type_possibly_after_sub() || base.is::<EnumType>() {
            // Key must be a String constant
            let Some(local_name) = local_name else {
                return Ok(None);
            };

            // Qualifier must be a compile-time Namespace, otherwise return static dynamic reference.
            if qual.as_ref().map(|q| q.is::<Namespace>()).unwrap_or(true) {
                let k = map_defer_error(PropertyLookupKey::LocalName(local_name).computed_or_local_name(self.0))?;
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
            let Some(key) = local_name else {
                return Ok(None);
            };

            // Qualifier must be a compile-time Namespace, otherwise return static dynamic reference.
            if qual.map(|q| q.is::<Namespace>()).unwrap_or(true) {
                let k = map_defer_error(PropertyLookupKey::LocalName(key).computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_static_dynamic_reference_value(base, qual, &k)));
            }

            return Ok(None);
        }

        // Base a little bit in https://github.com/hydroper-jet/privcompiler/blob/master/src/compiler/semantics/property_resolution.rs#L128
        // but read the semantics in the To Do list.

        // For a value
        if base.is::<Value>() {
            let base_type = defer(&base.static_type(self.0))?;
            let base_esc_type = base_type.escape_of_non_nullable();

            // If base is a value whose type is one of { XML, XML!, XMLList, XMLList! }, return a XML reference value.
            if [defer(&self.0.xml_type())?, defer(&self.0.xml_list_type())?].contains(&base_esc_type) {
                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_xml_reference_value(base, qual, &k)));
            }

            let has_known_ns = qual.as_ref().map(|q| q.is::<Namespace>()).unwrap_or(true);

            let Some(local_name) = local_name else {
                // Attempt to index Array
                if let Some(_) = map_defer_error(base_esc_type.array_element_type(self.0))? {
                    let iv: Option<Thingy> = TypeConversion(self.0).implicit(key.computed_or_local_name(self.0), &defer(&self.0.number_type())?, false);
                    if let Some(iv) = iv {
                        return Ok(Some(map_defer_error(self.0.factory().create_array_element_reference_value(&base, &iv))?));
                    }
                }

                // Attempt to index Vector
                if let Some(_) = map_defer_error(base_esc_type.vector_element_type(self.0))? {
                    let iv: Option<Thingy> = TypeConversion(self.0).implicit(key.computed_or_local_name(self.0), &defer(&self.0.number_type())?, false);
                    if let Some(iv) = iv {
                        return Ok(Some(map_defer_error(self.0.factory().create_vector_element_reference_value(&base, &iv))?));
                    }
                }
                
                // Attempt to index a tuple
                if double_key.is_some() && base_esc_type.is::<TupleType>() {
                    let index: usize = unsafe { double_key.unwrap().to_int_unchecked() };
                    if index >= base_type.element_types().length() {
                        return Ok(None);
                    }
                    return Ok(Some(self.0.factory().create_tuple_reference_value(&base, index)));
                }

                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
            };

            // If base data type is one of { *, Object, Object! }, or a dynamic class, or
            // if qualifier is not a compile-time control namespace,
            // return a dynamic reference value..
            let any_or_object = [self.0.any_type(), defer(&self.0.object_type())?].contains(&base_esc_type);
            if
                any_or_object
            || map_defer_error(base_type.escape_of_non_nullable().is_dynamic_or_inherits_dynamic(self.0))?
            || !has_known_ns
            {
                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
            }

            todo();

            return Ok(None);
        }

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