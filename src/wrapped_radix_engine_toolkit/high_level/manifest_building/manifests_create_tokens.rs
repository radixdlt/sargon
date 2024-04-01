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
        let scrypto_manifest = builder
            .try_deposit_entire_worktop_or_abort(
                address_of_owner.scrypto(),
                None,
            )
            .build();

        TransactionManifest::sargon_built(
            scrypto_manifest,
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

        let scrypto_manifest = builder
            .try_deposit_entire_worktop_or_abort(
                address_of_owner.scrypto(),
                None,
            )
            .build();

        TransactionManifest::sargon_built(
            scrypto_manifest,
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
    pub fn create_multiple_fungible_tokens(
        address_of_owner: &AccountAddress,
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
        let fungibles = multiple_fungibles.tokens;

        let mut builder = ScryptoManifestBuilder::new();

        for metadata in fungibles.iter() {
            builder = Self::create_fungible_token_with_metadata_without_deposit(
                builder,
                21_000_000.into(),
                metadata.clone(),
            );
        }

        let scrypto_manifest = builder
            .try_deposit_entire_worktop_or_abort(
                address_of_owner.scrypto(),
                None,
            )
            .build();

        TransactionManifest::sargon_built(
            scrypto_manifest,
            address_of_owner.network_id(),
        )
    }
}

impl TokenDefinitionMetadata {
    pub(crate) fn for_nft_collection(index: U11) -> Self {
        let word = bip39_word_by_index(index).word;
        let name = capitalize(word.clone());
        TokenDefinitionMetadata::new(
            name.clone(),
            format!("{}: An amazingly innovative and rare NFT collection", name),
            word.to_uppercase(),
            "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg",
            ["Unique".to_string(), "FOMO".to_string(), "Advanced".to_string()],
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
                    Enum<0u8>(
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
    Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
                2,
                2,
            );
        let expected_manifest = r##"
        CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
        Enum<2u8>(
            Enum<0u8>()
        )
        Enum<1u8>()
        true
        Enum<0u8>(
            Enum<0u8>(
                Tuple(
                    Array<Enum>(
                        Enum<14u8>(
                            Array<Enum>(
                                Enum<0u8>(
                                    12u8
                                )
                            )
                        )
                    ),
                    Array<Tuple>(
                        Tuple(
                            Enum<1u8>(
                                "NonFungibleTokenData"
                            ),
                            Enum<1u8>(
                                Enum<0u8>(
                                    Array<String>(
                                        "name"
                                    )
                                )
                            )
                        )
                    ),
                    Array<Enum>(
                        Enum<0u8>()
                    )
                )
            ),
            Enum<1u8>(
                0u64
            ),
            Array<String>(
                "name"
            )
        )
        Map<NonFungibleLocalId, Tuple>(
            NonFungibleLocalId("#0#") => Tuple(
                Tuple(
                    "nf-number-0"
                )
            ),
            NonFungibleLocalId("#1#") => Tuple(
                Tuple(
                    "nf-number-1"
                )
            )
        )
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
                            "Abandon: An amazingly innovative and rare NFT collection"
                        )
                    ),
                    false
                ),
                "icon_url" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>(
                            "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg"
                        )
                    ),
                    false
                ),
                "name" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>(
                            "Abandon"
                        )
                    ),
                    false
                ),
                "symbol" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>(
                            "ABANDON"
                        )
                    ),
                    false
                ),
                "tags" => Tuple(
                    Enum<1u8>(
                        Enum<128u8>(
                            Array<String>(
                                "Unique",
                                "FOMO",
                                "Advanced"
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
    CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
        Enum<2u8>(
            Enum<0u8>()
        )
        Enum<1u8>()
        true
        Enum<0u8>(
            Enum<0u8>(
                Tuple(
                    Array<Enum>(
                        Enum<14u8>(
                            Array<Enum>(
                                Enum<0u8>(
                                    12u8
                                )
                            )
                        )
                    ),
                    Array<Tuple>(
                        Tuple(
                            Enum<1u8>(
                                "NonFungibleTokenData"
                            ),
                            Enum<1u8>(
                                Enum<0u8>(
                                    Array<String>(
                                        "name"
                                    )
                                )
                            )
                        )
                    ),
                    Array<Enum>(
                        Enum<0u8>()
                    )
                )
            ),
            Enum<1u8>(
                0u64
            ),
            Array<String>(
                "name"
            )
        )
        Map<NonFungibleLocalId, Tuple>(
            NonFungibleLocalId("#0#") => Tuple(
                Tuple(
                    "nf-number-0"
                )
            ),
            NonFungibleLocalId("#1#") => Tuple(
                Tuple(
                    "nf-number-1"
                )
            )
        )
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
                            "Ability: An amazingly innovative and rare NFT collection"
                        )
                    ),
                    false
                ),
                "icon_url" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>(
                            "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg"
                        )
                    ),
                    false
                ),
                "name" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>(
                            "Ability"
                        )
                    ),
                    false
                ),
                "symbol" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>(
                            "ABILITY"
                        )
                    ),
                    false
                ),
                "tags" => Tuple(
                    Enum<1u8>(
                        Enum<128u8>(
                            Array<String>(
                                "Unique",
                                "FOMO",
                                "Advanced"
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
        Address("account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql")
        "try_deposit_batch_or_abort"
        Expression("ENTIRE_WORKTOP")
        Enum<0u8>()
    ;
        "##;
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
