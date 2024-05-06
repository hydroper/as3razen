use crate::ns::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypeConversionVariant {
    FromAny,

    ToAny,

    /// Implicit conversion between number types,
    /// where the base and target are as they are
    /// (not marked nullable or non-nullable).
    BetweenNumber,

    /// Implicit conversion to covariant type.
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

    /// Implicit conversion to `Object`.
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
    }

    pub fn implicit(&self, value: &Thingy, target_type: &Thingy, optional: bool) -> Result<Option<Thingy>, DeferError> {
    }

    pub fn explicit(&self, value: &Thingy, target_type: &Thingy, optional: bool) -> Result<Option<Thingy>, DeferError> {
    }
}