use crate::prelude::*;

/// Note: Current implementation only adds support for associated values of `MetadataStringValue` & `MetadataUrlValue` variants.
///
/// Will need to add support for remaining variants if we want to check its values.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, uniffi::Enum)]
#[serde(tag = "type")]
pub enum MetadataTypedValue {
    #[serde(rename = "Bool")]
    MetadataBoolValue {},
    #[serde(rename = "BoolArray")]
    MetadataBoolArrayValue {},
    #[serde(rename = "Decimal")]
    MetadataDecimalValue {},
    #[serde(rename = "DecimalArray")]
    MetadataDecimalArrayValue {},
    #[serde(rename = "GlobalAddress")]
    MetadataGlobalAddressValue {},
    #[serde(rename = "GlobalAddressArray")]
    MetadataGlobalAddressArrayValue {},
    #[serde(rename = "I32")]
    MetadataI32Value {},
    #[serde(rename = "I32Array")]
    MetadataI32ArrayValue {},
    #[serde(rename = "I64")]
    MetadataI64Value {},
    #[serde(rename = "I64Array")]
    MetadataI64ArrayValue {},
    #[serde(rename = "Instant")]
    MetadataInstantValue {},
    #[serde(rename = "InstantArray")]
    MetadataInstantArrayValue {},
    #[serde(rename = "NonFungibleGlobalId")]
    MetadataNonFungibleGlobalIdValue {},
    #[serde(rename = "NonFungibleGlobalIdArray")]
    MetadataNonFungibleGlobalIdArrayValue {},
    #[serde(rename = "NonFungibleLocalId")]
    MetadataNonFungibleLocalIdValue {},
    #[serde(rename = "NonFungibleLocalIdArray")]
    MetadataNonFungibleLocalIdArrayValue {},
    #[serde(rename = "Origin")]
    MetadataOriginValue {},
    #[serde(rename = "OriginArray")]
    MetadataOriginArrayValue {},
    #[serde(rename = "PublicKey")]
    MetadataPublicKeyValue {},
    #[serde(rename = "PublicKeyArray")]
    MetadataPublicKeyArrayValue {},
    #[serde(rename = "PublicKeyHash")]
    MetadataPublicKeyHashValue {},
    #[serde(rename = "PublicKeyHashArray")]
    MetadataPublicKeyHashArrayValue {},
    #[serde(rename = "String")]
    MetadataStringValue { value: String },
    #[serde(rename = "StringArray")]
    MetadataStringArrayValue {},
    #[serde(rename = "U32")]
    MetadataU32Value {},
    #[serde(rename = "U32Array")]
    MetadataU32ArrayValue {},
    #[serde(rename = "U64")]
    MetadataU64Value {},
    #[serde(rename = "U64Array")]
    MetadataU64ArrayValue {},
    #[serde(rename = "U8")]
    MetadataU8Value {},
    #[serde(rename = "U8Array")]
    MetadataU8ArrayValue {},
    #[serde(rename = "Url")]
    MetadataUrlValue { value: Url },
    #[serde(rename = "UrlArray")]
    MetadataUrlArrayValue {},
}
