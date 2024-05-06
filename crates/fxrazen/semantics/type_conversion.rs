use crate::ns::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypeConversionVariant {
    /// Implicit conversion.
    FromAny,

    /// Implicit conversion.
    ToAny,

    /// Implicit conversion between number types,
    /// where the base and target are as they are
    /// (not marked nullable or non-nullable).
    BetweenNumber,

    /// Implicit conversion to covariant type.
    /// 
    /// Involved types either both include null or both are marked *non-nullable*.
    /// 
    /// Base:
    ///
    /// * Contravariant type
    /// * Nullable contravariant type
    /// * Non-nullable contravariant type
    /// 
    /// Targets:
    /// 
    /// * Covariant type
    /// * Nullable covariant type
    /// * Non-nullable covariant type
    ToCovariant,

    /// Explicit conversion from `Object` to interface.
    ///
    /// Targets:
    ///
    /// * Interface
    ObjectToItrfc,

    /// Implicit conversion to `Object`, `Object?` or `Object!`.
    /// 
    /// Involved types either both include null or both are marked *non-nullable*.
    /// 
    /// Base:
    ///
    /// * Interface
    /// * Nullable interface
    /// * Non nullable interface
    ItrfcToObject,

    /// Implicit conversion.
    NonNullableToNullable,

    /// Implicit conversion to `T?` where the `T` is a type
    /// that includes `null` without having been
    /// marked explicitly nullable.
    AsIsToNullable,

    /// Implicit conversion from `T` to `T?` where `T` is a type
    /// that includes `null` without having been
    /// marked explicitly nullable.
    NullableToAsIs,

    /// Implicit conversion from `T!` to `T` where `T` is a type
    /// that includes `null` without having been
    /// marked explicitly nullable.
    NonNullableToAsIs,

    /// Explicit conversion.
    /// 
    /// Base:
    /// 
    /// * As-is
    /// * Nullable
    /// * Non-nullable
    /// 
    /// Targets:
    /// 
    /// * Contravariant type (not marked nullable or non-nullable)
    ToContravariant,

    /// Explicit conversion.
    /// 
    /// Base:
    /// 
    /// * `Vector.<B>`
    /// * `Vector.<B>?`
    /// * `Vector.<B>!`
    /// 
    /// Targets:
    /// 
    /// * `Vector.<A>`
    ToCovariantVector,

    /// Explicit conversion.
    StringToEnum,

    /// Explicit conversion.
    NumberToEnum,

    /// Explicit conversion.
    FromTypeParameter,

    /// Explicit conversion where the type arguments too
    /// a parameterized type are changed.
    ParameterizedTypeAlter,
}

pub struct TypeConversions<'a>(pub &'a SemanticHost);

impl<'a> TypeConversions<'a> {
    pub fn implicit_constant(&self, value: &Thingy, target_type: &Thingy) -> Result<Option<Thingy>, DeferError> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Ok(Some(value.clone()));
        }
        if !value.is::<Constant>() {
            return Ok(None);
        }

        // undefined to type containing undefined or null
        if value.is::<UndefinedConstant>() {
            if target_type.includes_undefined(self.0)? {
                return Ok(Some(self.0.factory().create_undefined_constant(target_type)));
            } else if target_type.includes_null(self.0)? {
                return Ok(Some(self.0.factory().create_null_constant(target_type)));
            }
        }

        // null to type containing undefined or null
        if value.is::<NullConstant>() && (target_type.includes_undefined(self.0)? || target_type.includes_null(self.0)?) {
            return Ok(Some(self.0.factory().create_null_constant(target_type)));
        }

        let object_type = self.0.object_type().defer()?;
        let target_esc_type = target_type.escape_of_nullable_or_non_nullable();

        // Number constant to *, Object or Object!
        if value.is::<NumberConstant>() && (target_type.is::<AnyType>() || target_esc_type == object_type) {
            return Ok(Some(self.0.factory().create_number_constant(value.number_value(), target_type)));
        }

        if value.is::<NumberConstant>() && self.0.numeric_types()?.contains(&target_esc_type) {
            let v = value.number_value().convert_type(target_type, self.0)?;
            return Ok(Some(self.0.factory().create_number_constant(v, target_type)));
        }

        // From T or T! constant to T?, or
        // from T or T? constant to T!
        if (target_type.is::<NullableType>() && target_type.base() == from_type.escape_of_nullable_or_non_nullable())
        || (target_type.is::<NonNullableType>() && target_type.base() == from_type.escape_of_nullable_or_non_nullable()) {
            let new_k = value.clone_constant(self.0);
            new_k.set_static_type(target_type.clone());
            return Ok(Some(new_k));
        }

        Ok(None)
    }

    pub fn implicit(&self, value: &Thingy, target_type: &Thingy, optional: bool) -> Result<Option<Thingy>, DeferError> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Ok(Some(value.clone()));
        }

        let kc = self.implicit_constant(value, target_type)?;
        if kc.is_some() {
            return Ok(kc);
        }

        // From *
        if from_type.is::<AnyType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::FromAny, optional, target_type)?));
        }

        // To *
        if target_type.is::<AnyType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::ToAny, optional, target_type)?));
        }

        // Between number types
        if self.0.numeric_types()?.contains(&from_type) && self.0.numeric_types()?.contains(&target_type) {
            return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::BetweenNumber, optional, target_type)?));
        }

        let from_type_esc = from_type.escape_of_nullable_or_non_nullable();
        let target_type_esc = target_type.escape_of_nullable_or_non_nullable();

        // Covariant
        if from_type_esc.is_subtype_of(&target_type_esc, self.0)? {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_non_nullable = from_type.is::<NonNullableType>() && target_type.is::<NonNullableType>();

            if both_include_null || both_non_nullable {
                return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::ToCovariant, optional, target_type)?));
            }
        }

        let object_type = self.0.object_type().defer()?;

        if target_type == &object_type && from_type_esc.is_interface_type_possibly_after_sub() {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_non_nullable = from_type.is::<NonNullableType>() && target_type.is::<NonNullableType>();

            // ItrfcToObject
            if both_include_null || both_non_nullable {
                return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::ItrfcToObject, optional, target_type)?));
            }
        }

        if target_type.is::<NullableType>() {
            // NonNullableToNullable
            if from_type.is::<NonNullableType>() && from_type.base() == target_type.base() {
                return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::NonNullableToNullable, optional, target_type)?));
            }

            // AsIsToNullable
            if from_type == target_type.base() {
                return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::AsIsToNullable, optional, target_type)?));
            }
        }

        // NullableToAsIs
        if from_type.is::<NullableType>() && target_type == &from_type.base() && target_type.includes_null(self.0)? {
            return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::NullableToAsIs, optional, target_type)?));
        }

        // NonNullableToAsIs
        if from_type.is::<NonNullableType>() && target_type == &from_type.base() {
            return Ok(Some(self.0.factory().create_conversion_value(value, TypeConversionVariant::NonNullableToAsIs, optional, target_type)?));
        }

        Ok(None)
    }

    pub fn explicit(&self, value: &Thingy, target_type: &Thingy, optional: bool) -> Result<Option<Thingy>, DeferError> {
    }
}