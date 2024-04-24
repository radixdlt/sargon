use crate::prelude::*;
use delegate::delegate;

/// Small macro to facilitate generation of UniFFI exported functions.
macro_rules! decl_exactly_n_bytes {
    (
        $(
            #[doc = $expr: expr]
        )*
        $byte_count:literal,
        $exp_sample_value:literal,
    ) => {
        paste! {

            #[derive(
                Zeroize, // Not `ZeroizeOnDrop`: we dont wanna zeroize all byte types: use `decl_secret_bytes!` for secrets.
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                Ord,
                PartialOrd,
            )]
            struct [<Exactly $byte_count Bytes SecretMagic>]([u8; $byte_count]);

            impl From<&[u8; $byte_count]> for BagOfBytes {
                fn from(value: &[u8; $byte_count]) -> BagOfBytes {
                    BagOfBytes::from(value.as_ref())
                }
            }

            impl [<Exactly $byte_count Bytes SecretMagic>] {
                pub fn bytes(&self) -> &[u8; $byte_count] {
                    &self.0
                }
                pub fn to_hex(self) -> String {
                    hex_encode(self.0)
                }
                pub fn to_vec(self) -> Vec<u8> {
                    self.bytes().to_vec()
                }
            }

            impl From<[u8; $byte_count]> for [<Exactly $byte_count Bytes SecretMagic>] {
                fn from(value: [u8; $byte_count]) -> Self {
                    Self(value)
                }
            }

            impl TryFrom<&[u8]> for [<Exactly $byte_count Bytes SecretMagic>] {
                type Error = crate::CommonError;
                fn try_from(value: &[u8]) -> Result<Self> {
                    if value.len() == $byte_count {
                        Ok(Self(value.try_into().unwrap()))
                    } else {
                        Err(CommonError::InvalidByteCount {
                            expected: $byte_count as u64,
                            found: value.len() as u64,
                        })
                    }
                }
            }

            uniffi::custom_type!([<Exactly $byte_count Bytes SecretMagic>], BagOfBytes);

            impl crate::UniffiCustomTypeConverter for [<Exactly $byte_count Bytes SecretMagic>] {
                type Builtin = BagOfBytes;

                #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    Self::try_from(val.as_ref()).map_err(|e| e.into())
                }

                #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
                fn from_custom(obj: Self) -> Self::Builtin {
                    BagOfBytes::from(obj.to_vec())
                }
            }

            $(
                #[doc = $expr]
            )*
            #[derive(
                Zeroize,
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                Ord,
                PartialOrd,
                SerializeDisplay,
                DeserializeFromStr,
                derive_more::Display,
                derive_more::Debug,
                uniffi::Record,
            )]
            #[display("{}", self.to_hex())]
            #[debug("{}", self.to_hex())]
            pub struct [<Exactly $byte_count Bytes>] {
                secret_magic: [<Exactly $byte_count Bytes SecretMagic>],
            }

            // Make it JSON String convertible in Swift/Kotlin
            json_string_convertible!([<Exactly $byte_count Bytes>]);

            impl From<[<Exactly $byte_count Bytes SecretMagic>]> for [<Exactly $byte_count Bytes>] {
                fn from(value: [<Exactly $byte_count Bytes SecretMagic>]) -> Self {
                    Self { secret_magic: value }
                }
            }

            impl FromStr for [<Exactly $byte_count Bytes>] {
                type Err = crate::CommonError;
                fn from_str(s: &str) -> Result<Self> {
                    BagOfBytes::from_str(s).and_then(|b| Self::try_from(b.as_ref()))
                }
            }

            impl [<Exactly $byte_count Bytes>] {
                pub fn bytes(&self) -> &[u8; $byte_count] {
                    self.secret_magic.bytes()
                }

                /// Instantiates a new `BagOfBytes` from bytes generated by
                /// a CSPRNG.
                pub fn generate() -> Self {
                    Self::from(&generate_byte_array::<$byte_count>())
                }

                /// Tries to decode the string `s` into this type. Will fail
                // if the string is not valid hex or if the decoded bytes does
                // not have length `N`.
                pub fn from_hex(s: &str) -> Result<Self> {
                   Self::from_str(s)
                }

                delegate! {
                    to self.secret_magic {
                        pub fn to_hex(self) -> String;
                        pub fn to_vec(self) -> Vec<u8>;
                    }
                }
            }


            impl TryFrom<&[u8]> for [<Exactly $byte_count Bytes>] {
                type Error = CommonError;

                fn try_from(value: &[u8]) -> Result<Self> {
                    [<Exactly $byte_count Bytes SecretMagic>]::try_from(value).map(Self::from)
                }
            }

            impl TryFrom<Vec<u8>> for [<Exactly $byte_count Bytes>] {
                type Error = CommonError;

                fn try_from(value: Vec<u8>) -> Result<Self> {
                    Self::try_from(value.as_slice())
                }
            }

            impl TryFrom<BagOfBytes> for [<Exactly $byte_count Bytes>] {
                type Error = CommonError;

                fn try_from(value: BagOfBytes) -> Result<Self> {
                    Self::try_from(value.as_ref())
                }
            }


            impl AsRef<[u8]> for [<Exactly $byte_count Bytes>] {
                fn as_ref(&self) -> &[u8] {
                    self.bytes()
                }
            }

            impl From<&[u8; $byte_count]> for [<Exactly $byte_count Bytes>] {
                fn from(value: &[u8; $byte_count]) -> Self {
                    Self::from([<Exactly $byte_count Bytes SecretMagic>]::from(*value))
                }
            }

            impl HasSampleValues for [<Exactly $byte_count Bytes>] {
                /// `deadbeef...``
                /// A sample used to facilitate unit tests.
                fn sample() -> Self {
                    Self::sample_dead()
                }

                /// A sample used to facilitate unit tests.
                fn sample_other() -> Self {
                   Self::sample_fade()
                }
            }

            impl [<Exactly $byte_count Bytes>] {

                fn declare_sample(r: &str) -> Self {
                    let mut s = r.repeat($byte_count / 2);
                    if s.len() != $byte_count * 2 {
                        s = format!("{}{}", $byte_count, s);
                    }
                    Self::from_str(&s).expect("Valid sample")
                }

                /// `aced...``
                /// A sample used to facilitate unit tests.
                pub fn sample_aced() -> Self {
                    Self::declare_sample("aced")
                }

                /// `babe...``
                /// A sample used to facilitate unit tests.
                pub fn sample_babe() -> Self {
                    Self::declare_sample("babe")
                }

                /// `cafe...``
                /// A sample used to facilitate unit tests.
                pub fn sample_cafe() -> Self {
                    Self::declare_sample("cafe")
                }

                /// `dead...``
                /// A sample used to facilitate unit tests.
                pub fn sample_dead() -> Self {
                    Self::declare_sample("dead")
                }

                /// `ecad...``
                /// A sample used to facilitate unit tests.
                pub fn sample_ecad() -> Self {
                    Self::declare_sample("ecad")
                }

                /// `fade...``
                /// A sample used to facilitate unit tests.
                pub fn sample_fade() -> Self {
                    Self::declare_sample("fade")
                }

            }

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes>](
                bytes: BagOfBytes,
            ) -> Result<[< Exactly $byte_count Bytes >]> {
                [< Exactly $byte_count Bytes >]::try_from(bytes)
            }

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes_sample>](
            ) -> [< Exactly $byte_count Bytes >] {
                [< Exactly $byte_count Bytes >]::sample()
            }

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes_sample_other>](
            ) -> [< Exactly $byte_count Bytes >] {
                [< Exactly $byte_count Bytes >]::sample_other()
            }

            #[uniffi::export]
            pub fn [<exactly_ $byte_count _bytes_to_bytes>](
                bytes: &[< Exactly $byte_count Bytes >],
            ) -> BagOfBytes {
                BagOfBytes::from(bytes.bytes())
            }

            #[uniffi::export]
            pub fn [<exactly_ $byte_count _bytes_to_hex>](
                bytes: &[< Exactly $byte_count Bytes >],
            ) -> String {
                bytes.to_hex()
            }

            #[cfg(test)]
            mod [<tests_ exactly_ $byte_count _bytes >] {

                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = [< Exactly $byte_count Bytes >];

                #[test]
                fn equality() {
                    assert_eq!(SUT::sample(), SUT::sample());
                    assert_eq!(SUT::sample_other(), SUT::sample_other());
                }

                #[test]
                fn inequality() {
                    assert_ne!(SUT::sample(), SUT::sample_other());
                }

                #[test]
                fn ord() {
                    assert!(SUT::sample() < SUT::sample_other());
                }

                #[test]
                fn hash() {
                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            SUT::sample_aced(),
                            SUT::sample_babe(),
                            SUT::sample_cafe(),
                            SUT::sample_dead(),
                            SUT::sample_ecad(),
                            SUT::sample_fade(),
                            // Duplicates should be removed
                            SUT::sample_aced(),
                            SUT::sample_babe(),
                            SUT::sample_cafe(),
                            SUT::sample_dead(),
                            SUT::sample_ecad(),
                            SUT::sample_fade(),
                        ])
                        .len(),
                        6
                    );
                }

                #[test]
                fn from_roundtrip() {
                    let bytes = &[0u8; $byte_count];
                    assert_eq!(SUT::from(bytes).bytes(), bytes);
                }

                #[test]
                fn from_vec_roundtrip() {
                    let vec = Vec::from([0u8; $byte_count]);
                    let sut: SUT = vec.clone().try_into().unwrap();
                    assert_eq!(sut.to_vec(), vec);
                }

                #[test]
                fn invalid_str() {
                    let s = "invalid str";
                    assert_eq!(
                        SUT::from_str(s),
                        Err(CommonError::StringNotHex {
                            bad_value: s.to_owned()
                        })
                    );
                }

                #[test]
                fn invalid_len() {
                    assert_eq!(
                        SUT::try_from(Vec::from([0u8; 5])),
                        Err(CommonError::InvalidByteCount {
                            expected: $byte_count,
                            found: 5
                        })
                    )
                }

                #[test]
                fn random() {
                    let mut set: HashSet<Vec<u8>> = HashSet::new();
                    let n = 100;
                    for _ in 0..n {
                        let bytes = SUT::generate();
                        set.insert(bytes.to_vec());
                    }
                    assert_eq!(set.len(), n);
                }

                #[test]
                fn manual_perform_uniffi_conversion_successful() {
                    let bytes = generate_byte_array::<$byte_count>();
                    let bag_of_bytes = BagOfBytes::from(&bytes);
                    let secret_magic = [<Exactly $byte_count Bytes SecretMagic>](bytes);

                    let ffi_side =
                        <[<Exactly $byte_count Bytes SecretMagic>] as crate::UniffiCustomTypeConverter>::from_custom(secret_magic);

                    assert_eq!(ffi_side, bag_of_bytes);

                      let from_ffi_side =  <[<Exactly $byte_count Bytes SecretMagic>] as crate::UniffiCustomTypeConverter>::into_custom(
                            bag_of_bytes,
                        )
                        .unwrap();
                    assert_eq!(secret_magic, from_ffi_side);
                }

                #[test]
                fn manual_perform_uniffi_conversion_fail() {
                    assert!(
                        <[<Exactly $byte_count Bytes SecretMagic>] as crate::UniffiCustomTypeConverter>::into_custom(
                            BagOfBytes::from(vec![0xde, 0xad]),
                        )
                        .is_err()
                    );
                }

                #[test]
                fn from_string_roundtrip() {
                    assert_eq!(SUT::from_hex($exp_sample_value).unwrap().to_string(), $exp_sample_value);
                }

                #[test]
                fn debug() {
                    let hex_bytes = SUT::sample();
                    assert_eq!(format!("{:?}", hex_bytes), $exp_sample_value);
                }

                #[test]
                fn display() {
                    let hex_bytes = SUT::sample();
                    assert_eq!(format!("{}", hex_bytes), $exp_sample_value);
                }

                #[test]
                fn to_hex() {
                    let hex_bytes = SUT::sample();
                    assert_eq!(hex_bytes.to_string(), $exp_sample_value);
                }

                #[test]
                fn as_ref() {
                    let b: &[u8] = &hex_decode(
                        $exp_sample_value
                    )
                    .unwrap();
                    assert_eq!(SUT::try_from(b).unwrap().as_ref(), b);
                }

                #[test]
                fn json_roundtrip() {
                    let model = SUT::sample();
                    assert_json_value_eq_after_roundtrip(
                        &model,
                        json!($exp_sample_value),
                    );
                }

                #[test]
                fn json_roundtrip_fails_for_invalid() {
                    assert_json_value_fails::<SUT>(json!("not even hex"));
                    // too short
                    assert_json_value_fails::<SUT>(json!("deadbeef"));
                    // too long
                    assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
                }
            }

            #[cfg(test)]
            mod [<uniffi_ tests_ exactly_ $byte_count _bytes>] {

                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = [< Exactly $byte_count Bytes >];

                #[test]
                fn new_from_bag_of_bytes() {
                    let bytes = generate_bytes::<$byte_count>();
                    assert_eq!(
                        [<new_exactly_ $byte_count _bytes>](bytes.clone().into()).unwrap().to_vec(),
                        bytes
                    );
                }

                #[test]
                fn new_fail() {
                    assert!([<new_exactly_ $byte_count _bytes>](generate_bytes::<5>().into()).is_err());
                }

                #[test]
                fn sample_values() {
                    assert_eq!(
                        [<new_exactly_ $byte_count _bytes_sample>](),
                        [<new_exactly_ $byte_count _bytes_sample>](),
                    );
                    assert_ne!(
                        [<new_exactly_ $byte_count _bytes_sample>](),
                        [<new_exactly_ $byte_count _bytes_sample_other>](),
                    );
                }

                #[test]
                fn to_bytes() {
                    let bytes = generate_byte_array::<$byte_count>();
                    let sut = SUT::from(&bytes);
                    assert_eq!(
                        [<exactly_ $byte_count _bytes_to_bytes>](&sut),
                        BagOfBytes::from(&bytes)
                    );
                }

                #[test]
                fn to_hex() {
                    let bytes = generate_byte_array::<$byte_count>();
                    let sut = SUT::from(&bytes);
                    assert_eq!(
                        [<exactly_ $byte_count _bytes_to_hex>](&sut),
                        hex_encode(&bytes)
                    );
                }
            }
        }
    };
}

decl_exactly_n_bytes!(
    /// 29 bytes, typically used as PublicKeyHash, or otherwise NodeId payload,
    /// implementation wise those bytes are stored inside a `BagOfBytes`
    /// (wrapper of `Vec<u8>`) for UniFFI compat.
    29,
    "29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 32 bytes, most commonly used fixed length bytes, used by PrivateKeys,
    /// Ed25519PublicKey, and BIP39 entropy, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    32,
    "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 64 bytes, used by Ed25519Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    64,
    "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 33 bytes, used by Secp256k1PublicKeys, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    33,
    "33deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 65 bytes, used by Secp256k1Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    65,
    "65deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 12 bytes, used by AES encryption, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    12,
    "deaddeaddeaddeaddeaddead", // expected sample value for tests
);
