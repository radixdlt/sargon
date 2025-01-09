use crate::prelude::*;

pub trait TransactionManifestTokenCreating {
    fn create_fungible_token_with_metadata_without_deposit(
        builder: ScryptoTransactionManifestBuilder,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> ScryptoTransactionManifestBuilder;

    fn create_non_fungible_tokens<T, V>(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        initial_supply: T,
    ) -> TransactionManifest
    where
        T: Clone + IntoIterator<Item = (NonFungibleLocalId, V)>,
        V: ScryptoManifestEncode + ScryptoNonFungibleData;

    fn create_fungible_token(
        address_of_owner: &AccountAddress,
    ) -> TransactionManifest;

    fn create_non_fungible_tokens_collections(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        nfts_per_collection: u64,
    ) -> TransactionManifest {
        Self::create_non_fungible_tokens_collections_with_local_id_fn(
            address_of_owner,
            collection_count,
            nfts_per_collection,
            |i| NonFungibleLocalId::Integer { value: i },
        )
    }

    fn create_non_fungible_tokens_collections_with_local_id_fn<F>(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        nfts_per_collection: u64,
        local_id: F,
    ) -> TransactionManifest
    where
        F: Fn(u64) -> NonFungibleLocalId;

    fn create_single_nft_collection(
        address_of_owner: &AccountAddress,
        nfts_per_collection: u64,
    ) -> TransactionManifest {
        Self::create_non_fungible_tokens_collections(
            address_of_owner,
            1,
            nfts_per_collection,
        )
    }

    fn create_multiple_nft_collections(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        nfts_per_collection: u64,
    ) -> TransactionManifest {
        Self::create_non_fungible_tokens_collections(
            address_of_owner,
            collection_count,
            nfts_per_collection,
        )
    }

    fn create_fungible_token_with_metadata(
        address_of_owner: &AccountAddress,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> TransactionManifest {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        builder = Self::create_fungible_token_with_metadata_without_deposit(
            builder,
            initial_supply,
            metadata,
        );
        builder = builder.try_deposit_entire_worktop_or_abort(
            address_of_owner.scrypto(),
            None,
        );

        TransactionManifest::sargon_built(
            builder,
            address_of_owner.network_id(),
        )
    }

    fn create_multiple_fungible_tokens(
        address_of_owner: &AccountAddress,
        count: impl Into<Option<u8>>,
    ) -> TransactionManifest;
}

impl TransactionManifestTokenCreating for TransactionManifest {
    fn create_fungible_token(
        address_of_owner: &AccountAddress,
    ) -> TransactionManifest {
        Self::create_fungible_token_with_metadata(
            address_of_owner,
            21_000_000.into(),
            TokenDefinitionMetadata::sample(),
        )
    }

    fn create_non_fungible_tokens_collections_with_local_id_fn<F>(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        nfts_per_collection: u64,
        local_id: F,
    ) -> Self
    where
        F: Fn(u64) -> NonFungibleLocalId,
    {
        Self::create_non_fungible_tokens(
            address_of_owner,
            collection_count,
            (0..nfts_per_collection)
                .map(|i| (local_id(i), NonFungibleTokenData::new(i))),
        )
    }

    fn create_non_fungible_tokens<T, V>(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        initial_supply: T,
    ) -> Self
    where
        T: Clone + IntoIterator<Item = (NonFungibleLocalId, V)>,
        V: ScryptoManifestEncode + ScryptoNonFungibleData,
    {
        if collection_count > U11::MAX {
            panic!("Must not be greater than {}", U11::MAX);
        }

        let mut builder = ScryptoTransactionManifestBuilder::new();

        let metadata_vec = (0..collection_count)
            .map(|i| U11::new(i).unwrap())
            .map(TokenDefinitionMetadata::for_nft_collection)
            .collect_vec();

        for metadata in metadata_vec.iter() {
            builder = builder.create_non_fungible_resource(
                ScryptoOwnerRole::Updatable(ScryptoAccessRule::AllowAll),
                ScryptoNonFungibleIdType::Integer,
                true,
                ScryptoNonFungibleResourceRoles::single_locked_rule(
                    ScryptoAccessRule::AllowAll,
                ),
                Into::<ScryptoModuleConfig<ScryptoMetadataInit>>::into(
                    metadata.clone(),
                ),
                Some(
                    initial_supply
                        .clone()
                        .into_iter()
                        .map(|t| (ScryptoNonFungibleLocalId::from(t.0), t.1))
                        .collect::<Vec<(ScryptoNonFungibleLocalId, V)>>(),
                ),
            )
        }

        builder = builder.try_deposit_entire_worktop_or_abort(
            address_of_owner.scrypto(),
            None,
        );

        TransactionManifest::sargon_built(
            builder,
            address_of_owner.network_id(),
        )
    }

    fn create_fungible_token_with_metadata_without_deposit(
        builder: ScryptoTransactionManifestBuilder,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> ScryptoTransactionManifestBuilder {
        let initial_supply: ScryptoDecimal192 = initial_supply.into();
        builder.create_fungible_resource(
            ScryptoOwnerRole::Updatable(ScryptoAccessRule::AllowAll),
            true,
            10,
            ScryptoFungibleResourceRoles::single_locked_rule(
                ScryptoAccessRule::AllowAll,
            ),
            metadata.into(),
            Some(initial_supply),
        )
    }

    /// Creates many fungible tokens, with initial supply, to be owned by `address_of_owner`.
    ///
    /// # Panics
    /// Panics if `address_of_owner` is on `Mainnet`, use a testnet instead.
    /// Panics if `count` is zero or is greater than the number of token metadata defined in `sample_resource_definition_metadata` (25)
    fn create_multiple_fungible_tokens(
        address_of_owner: &AccountAddress,
        count: impl Into<Option<u8>>,
    ) -> TransactionManifest {
        if address_of_owner.network_id() == NetworkID::Mainnet {
            panic!("To be 100% sure about license of the images, we do not allow these sample fungible tokens to be created on Mainnet.");
        }

        let json = serde_json::Value::from_str(include_str!(
            "sample_resource_definition_metadata.json"
        ))
        .expect("Should not have moved the metadata file.");

        #[derive(Deserialize)]
        struct MultipleFungibleTokens {
            description: String,
            tokens: Vec<TokenDefinitionMetadata>,
        }

        let multiple_fungibles: MultipleFungibleTokens =
            serde_json::from_value(json).unwrap();
        debug!("Generating multiple fungibles using bundled file, '\nDescription:\n'{}'", &multiple_fungibles.description);
        let all_fungibles = multiple_fungibles.tokens;
        let max_count = all_fungibles.len();
        let count = count.into().map(|c| c as usize).unwrap_or(max_count);
        if count > max_count || count == 0 {
            panic!(
                "Invalid 'count', must be: 0 < 'count' < {}",
                all_fungibles.len()
            )
        }
        let fungibles = all_fungibles.into_iter().take(count).collect_vec();
        assert_eq!(fungibles.len(), count);

        let mut builder = ScryptoTransactionManifestBuilder::new();

        for metadata in fungibles.iter() {
            builder = Self::create_fungible_token_with_metadata_without_deposit(
                builder,
                21_000_000.into(),
                metadata.clone(),
            );
        }

        builder = builder.try_deposit_entire_worktop_or_abort(
            address_of_owner.scrypto(),
            None,
        );

        TransactionManifest::sargon_built(
            builder,
            address_of_owner.network_id(),
        )
    }
}

impl TokenDefinitionMetadata {
    pub fn for_nft_collection(index: U11) -> Self {
        let word = bip39_word_by_index(index.clone()).word;
        let name = capitalize(word.clone());
        let base_url = "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/";

        let test_images = [
            "scryptonaut_patch.svg",
            "Filling+Station+Breakfast-large.jpg",
            "Filling+Station+Breakfast-medium.jpg",
            "Filling+Station+Breakfast-small.jpg",
            "Frame+6-large.png",
            "Frame+6-medium.png",
            "Frame+6-small.png",
            "Fried+Kway+Teow-large.jpg",
            "Fried+Kway+Teow-medium.jpg",
            "Fried+Kway+Teow-small.jpg",
            "ICON-transparency.png",
            "KL+Haze-large.jpg",
            "KL+Haze-medium.jpg",
            "KL+Haze-small.jpg",
            "modern_kunst_museum_pano-2.jpg",
            "modern_kunst_museum_pano-3.jpg",
            "modern_kunst_museum_pano.jpg",
        ];

        let test_image =
            test_images[(index.inner as usize) % test_images.len()];

        let icon_url = format!("{}{}", base_url, test_image);

        TokenDefinitionMetadata::new(
            name.clone(),
            format!(
                "{}: An amazingly innovative and rare NFT collection",
                name
            ),
            word.to_uppercase(),
            icon_url,
            [
                "Unique".to_string(),
                "FOMO".to_string(),
                "Advanced".to_string(),
            ],
        )
    }
}

#[derive(Clone, PartialEq, Eq, ScryptoSbor, ScryptoManifestSbor)]
struct NonFungibleTokenData {
    pub name: String,
}
impl NonFungibleTokenData {
    fn new(i: u64) -> Self {
        Self {
            name: format!("nf-number-{}", i),
        }
    }
}
impl ScryptoNonFungibleData for NonFungibleTokenData {
    const MUTABLE_FIELDS: &'static [&'static str] = &["name"];
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use prelude::fixture_rtm;
    use pretty_assertions::assert_eq;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn manifest_create_fungible_token_stella() {
        assert_eq!(
            SUT::create_fungible_token(&AccountAddress::sample_mainnet(),)
                .to_string(),
            r##"CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
    Enum<2u8>(
        Enum<0u8>()
    )
    true
    10u8
    Decimal("21000000")
    Tuple(
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<0u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<0u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<0u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<0u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<0u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<0u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        )
    )
    Tuple(
        Map<String, Tuple>(
            "description" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "The brightest component in the Radix ecosystem."
                    )
                ),
                false
            ),
            "extra_bool" => Tuple(
                Enum<1u8>(
                    Enum<1u8>(
                        true
                    )
                ),
                false
            ),
            "extra_bool_array" => Tuple(
                Enum<1u8>(
                    Enum<129u8>(
                        Array<Bool>(
                            true,
                            false
                        )
                    )
                ),
                false
            ),
            "extra_decimal" => Tuple(
                Enum<1u8>(
                    Enum<7u8>(
                        Decimal("8")
                    )
                ),
                false
            ),
            "extra_decimal_array" => Tuple(
                Enum<1u8>(
                    Enum<135u8>(
                        Array<Decimal>(
                            Decimal("1"),
                            Decimal("2")
                        )
                    )
                ),
                false
            ),
            "extra_global_address" => Tuple(
                Enum<1u8>(
                    Enum<8u8>(
                        Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                    )
                ),
                false
            ),
            "extra_global_address_array" => Tuple(
                Enum<1u8>(
                    Enum<136u8>(
                        Array<Address>(
                            Address("account_rdx128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4j0785p"),
                            Address("account_rdx12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9l00qnqm")
                        )
                    )
                ),
                false
            ),
            "extra_i32" => Tuple(
                Enum<1u8>(
                    Enum<5u8>(
                        32i32
                    )
                ),
                false
            ),
            "extra_i32_array" => Tuple(
                Enum<1u8>(
                    Enum<133u8>(
                        Array<I32>(
                            32i32,
                            33i32,
                            34i32,
                            35i32
                        )
                    )
                ),
                false
            ),
            "extra_i64" => Tuple(
                Enum<1u8>(
                    Enum<6u8>(
                        64i64
                    )
                ),
                false
            ),
            "extra_i64_array" => Tuple(
                Enum<1u8>(
                    Enum<134u8>(
                        Array<I64>(
                            64i64,
                            65i64,
                            66i64,
                            67i64
                        )
                    )
                ),
                false
            ),
            "extra_instant" => Tuple(
                Enum<1u8>(
                    Enum<12u8>(
                        1891i64
                    )
                ),
                false
            ),
            "extra_instant_array" => Tuple(
                Enum<1u8>(
                    Enum<140u8>(
                        Array<I64>(
                            5i64,
                            1891i64
                        )
                    )
                ),
                false
            ),
            "extra_non_fungible_global_id" => Tuple(
                Enum<1u8>(
                    Enum<10u8>(
                        NonFungibleGlobalId("resource_rdx1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9fty5xkn:#1#")
                    )
                ),
                false
            ),
            "extra_non_fungible_global_id_array" => Tuple(
                Enum<1u8>(
                    Enum<138u8>(
                        Array<Tuple>(
                            NonFungibleGlobalId("resource_rdx1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9fty5xkn:#1#"),
                            NonFungibleGlobalId("resource_rdx1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9fty5xkn:#2#")
                        )
                    )
                ),
                false
            ),
            "extra_non_fungible_local_id" => Tuple(
                Enum<1u8>(
                    Enum<11u8>(
                        NonFungibleLocalId("#1#")
                    )
                ),
                false
            ),
            "extra_non_fungible_local_id_array" => Tuple(
                Enum<1u8>(
                    Enum<139u8>(
                        Array<NonFungibleLocalId>(
                            NonFungibleLocalId("#1#"),
                            NonFungibleLocalId("#2#")
                        )
                    )
                ),
                false
            ),
            "extra_origin" => Tuple(
                Enum<1u8>(
                    Enum<14u8>(
                        "https://radixdlt.com"
                    )
                ),
                false
            ),
            "extra_origin_array" => Tuple(
                Enum<1u8>(
                    Enum<142u8>(
                        Array<String>(
                            "https://radixdlt.com",
                            "https://ociswap.com"
                        )
                    )
                ),
                false
            ),
            "extra_public_key" => Tuple(
                Enum<1u8>(
                    Enum<9u8>(
                        Enum<1u8>(
                            Bytes("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf")
                        )
                    )
                ),
                false
            ),
            "extra_public_key_array" => Tuple(
                Enum<1u8>(
                    Enum<137u8>(
                        Array<Enum>(
                            Enum<1u8>(
                                Bytes("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf")
                            ),
                            Enum<0u8>(
                                Bytes("033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8")
                            )
                        )
                    )
                ),
                false
            ),
            "extra_public_key_hash" => Tuple(
                Enum<1u8>(
                    Enum<15u8>(
                        Enum<1u8>(
                            Bytes("f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
                        )
                    )
                ),
                false
            ),
            "extra_public_key_hash_array" => Tuple(
                Enum<1u8>(
                    Enum<143u8>(
                        Array<Enum>(
                            Enum<1u8>(
                                Bytes("f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
                            ),
                            Enum<0u8>(
                                Bytes("4a5004504dbbc08c65ba86fcd7592a3ac48db81d217fe2356e75b37f31")
                            )
                        )
                    )
                ),
                false
            ),
            "extra_string" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "foo bar"
                    )
                ),
                false
            ),
            "extra_string_array" => Tuple(
                Enum<1u8>(
                    Enum<128u8>(
                        Array<String>(
                            "foo",
                            "bar"
                        )
                    )
                ),
                false
            ),
            "extra_u32" => Tuple(
                Enum<1u8>(
                    Enum<3u8>(
                        32u32
                    )
                ),
                false
            ),
            "extra_u32_array" => Tuple(
                Enum<1u8>(
                    Enum<131u8>(
                        Array<U32>(
                            32u32,
                            33u32,
                            34u32,
                            35u32
                        )
                    )
                ),
                false
            ),
            "extra_u64" => Tuple(
                Enum<1u8>(
                    Enum<4u8>(
                        64u64
                    )
                ),
                false
            ),
            "extra_u64_array" => Tuple(
                Enum<1u8>(
                    Enum<132u8>(
                        Array<U64>(
                            64u64,
                            65u64,
                            66u64,
                            67u64
                        )
                    )
                ),
                false
            ),
            "extra_u8" => Tuple(
                Enum<1u8>(
                    Enum<2u8>(
                        8u8
                    )
                ),
                false
            ),
            "extra_u8_array" => Tuple(
                Enum<1u8>(
                    Enum<130u8>(
                        Bytes("08090a0b")
                    )
                ),
                false
            ),
            "extra_url" => Tuple(
                Enum<1u8>(
                    Enum<13u8>(
                        "https://radixdlt.com"
                    )
                ),
                false
            ),
            "extra_url_array" => Tuple(
                Enum<1u8>(
                    Enum<141u8>(
                        Array<String>(
                            "https://radixdlt.com",
                            "https://ociswap.com"
                        )
                    )
                ),
                false
            ),
            "icon_url" => Tuple(
                Enum<1u8>(
                    Enum<13u8>(
                        "https://uxwing.com/wp-content/themes/uxwing/download/arts-graphic-shapes/star-full-icon.png"
                    )
                ),
                false
            ),
            "name" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "Stella"
                    )
                ),
                false
            ),
            "symbol" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "STAR"
                    )
                ),
                false
            ),
            "tags" => Tuple(
                Enum<1u8>(
                    Enum<128u8>(
                        Array<String>(
                            "Bright"
                        )
                    )
                ),
                false
            )
        ),
        Map<String, Enum>()
    )
    Enum<0u8>()
;
CALL_METHOD
    Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
    "try_deposit_batch_or_abort"
    Expression("ENTIRE_WORKTOP")
    Enum<0u8>()
;
"##
        );
    }

    #[test]
    fn create_multiple_fungible_tokens() {
        let manifest = TransactionManifest::create_multiple_fungible_tokens(
            &AccountAddress::sample_stokenet(),
            None,
        );
        assert_eq!(manifest.instructions().len(), 26);
    }

    #[test]
    #[should_panic(
        expected = "To be 100% sure about license of the images, we do not allow these sample fungible tokens to be created on Mainnet."
    )]
    fn create_multiple_fungible_tokens_panics_for_mainnet() {
        TransactionManifest::create_multiple_fungible_tokens(
            &AccountAddress::sample_mainnet(),
            None,
        );
    }

    #[test]
    #[should_panic(expected = "Invalid 'count', must be: 0 < 'count' < 25")]
    fn create_multiple_fungible_tokens_panics_when_count_is_too_large() {
        TransactionManifest::create_multiple_fungible_tokens(
            &AccountAddress::sample_stokenet(),
            100,
        );
    }

    #[test]
    #[should_panic(expected = "Invalid 'count', must be: 0 < 'count' < 25")]
    fn create_multiple_fungible_tokens_panics_when_count_is_zero() {
        TransactionManifest::create_multiple_fungible_tokens(
            &AccountAddress::sample_stokenet(),
            0,
        );
    }

    #[test]
    fn create_many_nft_collections() {
        let do_test = |n: u16| {
            let manifest =
                TransactionManifest::create_non_fungible_tokens_collections(
                    &AccountAddress::sample_stokenet(),
                    n,
                    3,
                );
            assert_eq!(manifest.instructions().len(), n as usize + 1);
            assert!(manifest.to_string().len() > 3000 * n as usize);
        };
        do_test(1);
        do_test(2);
        do_test(10);
    }

    #[test]
    fn create_two_nft_collections_assert_manifest() {
        let manifest =
            TransactionManifest::create_non_fungible_tokens_collections(
                &AccountAddress::sample_stokenet(),
                3,
                2,
            );
        let expected_manifest = fixture_rtm!("create_3_nft_collections");
        manifest_eq(manifest, expected_manifest);
    }

    #[test]
    #[should_panic(expected = "Must not be greater than 2047")]
    fn create_non_fungible_tokens_panics_if_collection_count_greater_than_max()
    {
        _ = SUT::create_non_fungible_tokens_collections(
            &AccountAddress::sample(),
            2048,
            1,
        );
    }
}
