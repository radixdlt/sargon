use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;

/// Small macro to facilitate generation of UniFFI exported functions.
macro_rules! decl_exactly_n_bytes {
    (
        $(
            #[doc = $expr: expr]
        )*
        $byte_count:literal
    ) => {
        paste! {
            use sargon::[<Exactly $byte_count Bytes>] as [<InternalExactly $byte_count Bytes>];

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                PartialEq,
                Eq,
                Hash,
                InternalConversion,
                 uniffi::Record,
            )]
            pub struct [<Exactly $byte_count Bytes>] {
                value: BagOfBytes,
            }

            impl From<[<InternalExactly $byte_count Bytes>]> for [<Exactly $byte_count Bytes>] {
                fn from(value: [<InternalExactly $byte_count Bytes>]) -> Self {
                    Self {
                        value: value.to_vec().into(),
                    }
                }
            }

            impl Into<[<InternalExactly $byte_count Bytes>]> for [<Exactly $byte_count Bytes>] {
                fn into(self) -> [<InternalExactly $byte_count Bytes>] {
                    [<InternalExactly $byte_count Bytes>]::try_from(self.value.into_internal()).unwrap()
                }
            }

            delegate_display_debug_into!([<Exactly $byte_count Bytes>], [<InternalExactly $byte_count Bytes>]);

            // Make it JSON String convertible in Swift/Kotlin
            json_string_convertible!([<Exactly $byte_count Bytes>]);

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes>](
                bytes: BagOfBytes,
            ) -> Result<[< Exactly $byte_count Bytes >]> {
                [<InternalExactly $byte_count Bytes>]::try_from(bytes.into_internal()).map_result()
            }

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes_sample>](
            ) -> [< Exactly $byte_count Bytes >] {
                [< InternalExactly $byte_count Bytes >]::sample().into()
            }

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes_sample_other>](
            ) -> [< Exactly $byte_count Bytes >] {
                [< InternalExactly $byte_count Bytes >]::sample_other().into()
            }

            #[uniffi::export]
            pub fn [<exactly_ $byte_count _bytes_to_bytes>](
                bytes: &[< Exactly $byte_count Bytes >],
            ) -> BagOfBytes {
                bytes.value.clone()
            }

            #[uniffi::export]
            pub fn [<exactly_ $byte_count _bytes_to_hex>](
                bytes: &[< Exactly $byte_count Bytes >],
            ) -> String {
                bytes.into_internal().to_hex()
            }
        }
    };
}

decl_exactly_n_bytes!(
    /// 29 bytes, typically used as PublicKeyHash, or otherwise NodeId payload,
    /// implementation wise those bytes are stored inside a `BagOfBytes`
    /// (wrapper of `Vec<u8>`) for UniFFI compat.
    29
);

decl_exactly_n_bytes!(
    /// 32 bytes, most commonly used fixed length bytes, used by PrivateKeys,
    /// Ed25519PublicKey, and BIP39 entropy, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    32
);

decl_exactly_n_bytes!(
    /// 64 bytes, used by Ed25519Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    64
);

decl_exactly_n_bytes!(
    /// 33 bytes, used by Secp256k1PublicKeys, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    33
);

decl_exactly_n_bytes!(
    /// 65 bytes, used by Secp256k1Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    65
);

decl_exactly_n_bytes!(
    /// 12 bytes, used by AES encryption, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    12
);

decl_exactly_n_bytes!(
    /// 60 bytes, used as encrypted mnemonic for security questions factor
    /// source. 32 bytes mnemonic when encrypted results in exactly this length.
    60
);
