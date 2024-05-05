use crate::ns::*;

pub struct PropertyLookup<'a>(pub &'a SemanticHost);

#[derive(Clone)]
pub enum PropertyLookupKey {
    String(String),
    Number(f64),
    Value(Thingy),
}

impl PropertyLookupKey {
    pub fn symbol(&self, host: &SemanticHost) -> Result<Thingy, DeferError> {
        match self {
            Self::String(s) => {
                let string_type = host.string_type().defer()?;
                Ok(host.factory().create_string_constant(s.clone(), &string_type))
            },
            Self::Number(d) => {
                let number_type = host.number_type().defer()?;
                Ok(host.factory().create_number_constant(NumberVariant::Number(d.clone()), &number_type))
            },
            Self::Value(s) => Ok(s.clone()),
        }
    }

    pub fn static_type(&self, host: &SemanticHost) -> Result<Thingy, DeferError> {
        match self {
            Self::String(_) => host.string_type().defer(),
            Self::Number(_) => host.number_type().defer(),
            Self::Value(s) => s.static_type(host).defer(),
        }
    }

    pub fn string_value(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            Self::Value(s) => {
                if s.is::<StringConstant>() {
                    Some(s.string_value())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn double_value(&self, host: &SemanticHost) -> Result<Option<f64>, DeferError> {
        Ok(match self {
            Self::Number(d) => Some(*d),
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