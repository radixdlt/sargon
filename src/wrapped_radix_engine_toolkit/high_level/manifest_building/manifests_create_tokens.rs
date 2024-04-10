use crate::prelude::*;

impl TransactionManifest {
    pub fn create_fungible_token(address_of_owner: &AccountAddress) -> Self {
        Self::create_fungible_token_with_metadata(
            address_of_owner,
            21_000_000.into(),
            TokenDefinitionMetadata::sample(),
        )
    }

    pub fn create_fungible_token_with_metadata(
        address_of_owner: &AccountAddress,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();
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

    pub fn create_single_nft_collection(
        address_of_owner: &AccountAddress,
        nfts_per_collection: u64,
    ) -> Self {
        Self::create_non_fungible_tokens_collections(
            address_of_owner,
            1,
            nfts_per_collection,
        )
    }

    pub fn create_multiple_nft_collections(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        nfts_per_collection: u64,
    ) -> Self {
        Self::create_non_fungible_tokens_collections(
            address_of_owner,
            collection_count,
            nfts_per_collection,
        )
    }

    fn create_non_fungible_tokens_collections(
        address_of_owner: &AccountAddress,
        collection_count: u16,
        nfts_per_collection: u64,
    ) -> Self {
        Self::create_non_fungible_tokens_collections_with_local_id_fn(
            address_of_owner,
            collection_count,
            nfts_per_collection,
            |i| NonFungibleLocalId::Integer { value: i },
        )
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested
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

    #[cfg(not(tarpaulin_include))] // false negative, tested
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

        let mut builder = ScryptoManifestBuilder::new();

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
                Into::<
                    radix_engine::types::node_modules::ModuleConfig<
                        radix_engine::types::MetadataInit,
                    >,
                >::into(metadata.clone()),
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

    pub fn create_fungible_token_with_metadata_without_deposit(
        builder: ScryptoManifestBuilder,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> ScryptoManifestBuilder {
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
    pub fn create_multiple_fungible_tokens(
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
        info!("Generating multiple fungibles using bundled file, '\nDescription:\n'{}'", &multiple_fungibles.description);
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

        let mut builder = ScryptoManifestBuilder::new();

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
    pub(crate) fn for_nft_collection(index: U11) -> Self {
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
            test_images[(index.inner.clone() as usize) % test_images.len()];

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
    use pretty_assertions::{assert_eq, assert_ne};
    use rand::Rng;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn manifest_create_fungible_token_stella() {
        assert_eq!(
            SUT::create_fungible_token(&AccountAddress::sample_mainnet(),)
                .to_string(),
            r#"CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
"#
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
        let expected_manifest = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_3_nft_collections.rtm"
        ));
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
