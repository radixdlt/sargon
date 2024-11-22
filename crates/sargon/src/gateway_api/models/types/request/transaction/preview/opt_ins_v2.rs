use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct TransactionPreviewRequestOptInsV2 {
    /**
     * This flag controls whether the preview response will include a Core API receipt or not. 
     * If not provided, this defaults to false and no core api receipt is provided in the response.
     */
    pub core_api_receipt: bool,

    /** 
     * This flag controls whether the preview response will include a Radix Engine Toolkit serializable receipt or not. 
     * If not provided, this defaults to false and no toolkit receipt is provided in the response.
     */
    pub radix_engine_toolkit_receipt: bool,
    
    /**
     * This flag controls whether the preview response will include execution logs.
     *  If not provided, this defaults to false and no logs will be provided in the response.
     */
    pub logs: bool
}
