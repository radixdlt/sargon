use crate::*;
use serde::{Deserialize, Serialize};

/// ProgrammaticScryptoSborValue : Arbitrary SBOR value represented as programmatic JSON with optional property name annotations.  All scalar types (`Bool`, `I*`, `U*`, `String`, `Reference`, `Own`, `Decimal`, `PreciseDecimal`, `NonFungibleLocalId`) convey their value via `value` string property with notable exception of `Bool` type that uses regular JSON boolean type. Numeric values as string-encoded to preserve accuracy and simplify implementation on platforms with no native support for 64-bit long numerical values.  Common properties represented as nullable strings:   * `type_name` is only output when a schema is present and the type has a name,   * `field_name` is only output when the value is a child of a `Tuple` or `Enum`, which has a type with named fields,   * `variant_name` is only output when a schema is present and the type is an `Enum`.  The following is a non-normative example annotated `Tuple` value with `String` and `U32` fields: ``` {   \"kind\": \"Tuple\",   \"type_name\": \"CustomStructure\",   \"fields\": [     {       \"kind\": \"String\",       \"field_name\": \"favorite_color\",       \"value\": \"Blue\"     },     {       \"kind\": \"U32\",       \"field_name\": \"usage_counter\",       \"value\": \"462231\"     }   ] } ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ProgrammaticScryptoSborValue {
    Bool(ProgrammaticScryptoSborValueBool),

    I8(ProgrammaticScryptoSborValueI8),

    I16(ProgrammaticScryptoSborValueI16),

    I32(ProgrammaticScryptoSborValueI32),

    I64(ProgrammaticScryptoSborValueI64),

    I128(ProgrammaticScryptoSborValueI128),

    U8(ProgrammaticScryptoSborValueU8),

    U16(ProgrammaticScryptoSborValueU16),

    U32(ProgrammaticScryptoSborValueU32),

    U64(ProgrammaticScryptoSborValueU64),

    U128(ProgrammaticScryptoSborValueU128),

    String(ProgrammaticScryptoSborValueString),

    Enum(ProgrammaticScryptoSborValueEnum),

    Array(ProgrammaticScryptoSborValueArray),

    Bytes(ProgrammaticScryptoSborValueBytes),

    Map(ProgrammaticScryptoSborValueMap),

    Tuple(ProgrammaticScryptoSborValueTuple),

    Reference(ProgrammaticScryptoSborValueReference),

    Own(ProgrammaticScryptoSborValueOwn),

    Decimal(ProgrammaticScryptoSborValueDecimal),

    PreciseDecimal(ProgrammaticScryptoSborValuePreciseDecimal),

    NonFungibleLocalId(ProgrammaticScryptoSborValueNonFungibleLocalId),
}

impl Default for ProgrammaticScryptoSborValue {
    fn default() -> Self {
        Self::Bool(Default::default())
    }
}
