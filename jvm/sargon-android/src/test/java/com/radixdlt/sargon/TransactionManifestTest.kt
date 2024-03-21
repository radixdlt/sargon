package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.createFungibleToken
import com.radixdlt.sargon.extensions.createFungibleTokenWithMetadata
import com.radixdlt.sargon.extensions.createMultipleFungibleTokens
import com.radixdlt.sargon.extensions.createMultipleNonFungibleTokens
import com.radixdlt.sargon.extensions.createNonFungibleToken
import com.radixdlt.sargon.extensions.faucet
import com.radixdlt.sargon.extensions.fromInt
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.markingAccountAsDAppDefinitionType
import com.radixdlt.sargon.extensions.modifyAddGuarantees
import com.radixdlt.sargon.extensions.modifyLockFee
import com.radixdlt.sargon.extensions.perAssetTransfers
import com.radixdlt.sargon.extensions.perRecipientTransfers
import com.radixdlt.sargon.extensions.setOwnerKeysHashes
import com.radixdlt.sargon.extensions.stakesClaim
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.thirdPartyDepositUpdate
import com.radixdlt.sargon.extensions.toDecimal192
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Disabled
import org.junit.jupiter.api.Test

class TransactionManifestTest: SampleTestable<TransactionManifest> {

    override val samples: List<Sample<TransactionManifest>>
        get() = listOf(TransactionManifest.sample)

    @Test
    fun test() {
        val instructionsString = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
        
        """.trimIndent()

        assertEquals(TransactionManifest.sample().string, instructionsString)

        val manifest = TransactionManifest.init(
            instructionsString = instructionsString,
            networkId = NetworkId.MAINNET
        )

        assertEquals(TransactionManifest.sample(), manifest)
    }

    @Test
    fun testCreateFungibleToken() {
        val instructionsString = """
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
            
        """.trimIndent()
        val manifest = TransactionManifest.createFungibleToken(
            AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testCreateNonFungibleToken() {
        val instructionsString = """
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
                        )
                    ),
                    NonFungibleLocalId("#10#") => Tuple(
                        Tuple(
                            "nf-number-10"
                        )
                    ),
                    NonFungibleLocalId("#11#") => Tuple(
                        Tuple(
                            "nf-number-11"
                        )
                    ),
                    NonFungibleLocalId("#12#") => Tuple(
                        Tuple(
                            "nf-number-12"
                        )
                    ),
                    NonFungibleLocalId("#13#") => Tuple(
                        Tuple(
                            "nf-number-13"
                        )
                    ),
                    NonFungibleLocalId("#14#") => Tuple(
                        Tuple(
                            "nf-number-14"
                        )
                    ),
                    NonFungibleLocalId("#15#") => Tuple(
                        Tuple(
                            "nf-number-15"
                        )
                    ),
                    NonFungibleLocalId("#16#") => Tuple(
                        Tuple(
                            "nf-number-16"
                        )
                    ),
                    NonFungibleLocalId("#17#") => Tuple(
                        Tuple(
                            "nf-number-17"
                        )
                    ),
                    NonFungibleLocalId("#18#") => Tuple(
                        Tuple(
                            "nf-number-18"
                        )
                    ),
                    NonFungibleLocalId("#19#") => Tuple(
                        Tuple(
                            "nf-number-19"
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
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "try_deposit_batch_or_abort"
                Expression("ENTIRE_WORKTOP")
                Enum<0u8>()
            ;
            
        """.trimIndent()
        val manifest = TransactionManifest.createNonFungibleToken(
            AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testCreateFungibleTokenWithMetadata() {
        val instructionsString = """
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
                Enum<2u8>(
                    Enum<0u8>()
                )
                true
                10u8
                Decimal("100")
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
                                    "Test fungible"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://www.no-icon.com"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Test"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "TST"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "test"
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

        """.trimIndent()
        val manifest = TransactionManifest.createFungibleTokenWithMetadata(
            addressOfOwner = AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            initialSupply = 100.toDecimal192(),
            metadata = TokenDefinitionMetadata(
                name = "Test",
                description = "Test fungible",
                symbol = "TST",
                iconUrl = "https://www.no-icon.com",
                tags = listOf("test")
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testCreateMultipleFungibleTokens() {
        val instructionsString = """
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Join the purr-fect revolution in digital currency with Purrcoin! It's the cat's meow in the world of blockchain."
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/animals-and-birds/cat-face-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Purrcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "PURR"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Dive into the quacky world of crypto with Quackcoin! It's making waves and turning heads faster than you can say 'duck'!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/animals-and-birds/duck-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Quackcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "QUACK"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Spacecake - fueling intergalactic transactions with a sprinkle of sweetness! It's out of this world!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/food-and-drinks/cake-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Spacecake"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "CAKE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Pawscoin - the paw-some digital currency that leaves a lasting impression with every transaction!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/animals-and-birds/paw-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Pawscoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "PAWS"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Bitecoin - taking a big bite out of the cryptocurrency market! Sink your teeth into profits with Bitecoin today."
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/animals-and-birds/shark-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Bitecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "BITE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Jellycoin - making your investments wobble with excitement! Dive into a pool of sweet profits."
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/animals-and-birds/jellyfish-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Jellycoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "JELLY"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Rocketfuel - powering your investments to new heights! Strap in for a journey to the moon and beyond."
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/transportation-automotive/rocket-launch-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Rocketfuel"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "FUEL"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Blisscoin - experience pure bliss with every transaction! Happiness guaranteed in the world of digital currency."
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/emoji-emoticon/laughing-black-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Blisscoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "BLISS"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Glowcoin - illuminating the path to financial freedom with its radiant transactions!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/arts-graphic-shapes/glare-star-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Glowcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "GLOW"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Scribblecoin - making its mark in the world of digital currency, one transaction at a time!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/editing-user-action/pencil-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Scribblecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "SCRIB"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Whirlwindcoin - swirling through the cryptocurrency market with unstoppable force! Hold on tight for the ride!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/weather/hurricane-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Whirlwindcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "WHIRL"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Frostycoin - cooling down the cryptocurrency market with its chilly transactions! Stay frosty, investors!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/weather/cooling-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Frostycoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "FROST"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Thundercoin - rumbling through the cryptocurrency market with electrifying transactions! Feel the power!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/editing-user-action/spark-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Thundercoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "THUNDER"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Hootcoin - wise investments for the nocturnal investor! Join the night owls of the crypto world!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/animals-and-birds/owl-bird-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Hootcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "HOOT"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Gigawattcoin - electrifying the cryptocurrency market with its powerful transactions! Shockingly good investments!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/household-and-furniture/electric-plugin-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Gigawattcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "GIGAWATT"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Sproutcoin - growing profits in the cryptocurrency market like a sprout in fertile soil! Plant your investments!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/nature-and-environment/flower-plant-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Sproutcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "SPROUT"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Whistlecoin - blowing the whistle on profits in the cryptocurrency market! Listen closely for success!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/sport-and-awards/whistle-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Whistlecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "WHISTLE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Jestercoin - adding a touch of whimsy to the cryptocurrency market with its playful transactions! Invest with a smile!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/toys-childhood/joker-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Jestercoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "JESTER"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Lighthousecoin - guiding investors safely through the cryptocurrency market with its beacon-like transactions! Navigate with confidence!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/buildings-architecture-real-estate/lighthouse-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Lighthousecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "LIGHTHOUSE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Peppercoin - adding a dash of spice to the cryptocurrency market with its zesty transactions! Invest with flavor!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/fruits-vegetables/chili-vegetable-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Peppercoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "PEPPER"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Dunecoin - carving a path through the cryptocurrency market like shifting sands in the desert! Navigate with precision!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/buildings-architecture-real-estate/sand-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Dunecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "DUNE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Pinnaclecoin - reaching the peak of success in the cryptocurrency market with its towering transactions! Aim high!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/nature-and-environment/peak-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Pinnaclecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "PINNACLE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Cometcoin - streaking through the cryptocurrency market like a shooting star of success! Blaze a trail!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/nature-and-environment/asteroid-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Cometcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "COMET"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Tidecoin - ebbing and flowing with profits in the cryptocurrency market like the tide of success! Ride the wave!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/nature-and-environment/ocean-waves-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Tidecoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "TIDE"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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
            CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
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
                                    "Zestcoin - adding a zesty flavor to the cryptocurrency market with its tangy transactions! Savor the taste of success!"
                                )
                            ),
                            false
                        ),
                        "icon_url" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "https://uxwing.com/wp-content/themes/uxwing/download/fruits-vegetables/orange-lemon-icon.png"
                                )
                            ),
                            false
                        ),
                        "name" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "Zestcoin"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ZEST"
                                )
                            ),
                            false
                        ),
                        "tags" => Tuple(
                            Enum<1u8>(
                                Enum<128u8>(
                                    Array<String>(
                                        "Sargon",
                                        "Rusty"
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

        """.trimIndent()
        val manifest = TransactionManifest.createMultipleFungibleTokens(
            addressOfOwner = AccountAddress.init(
                "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql"
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testCreateMultipleNonFungibleTokens() {
        val instructionsString = """
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Able: An amazingly innovative and rare NFT collection"
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
                                    "Able"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABLE"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "About: An amazingly innovative and rare NFT collection"
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
                                    "About"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABOUT"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Above: An amazingly innovative and rare NFT collection"
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
                                    "Above"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABOVE"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Absent: An amazingly innovative and rare NFT collection"
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
                                    "Absent"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABSENT"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Absorb: An amazingly innovative and rare NFT collection"
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
                                    "Absorb"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABSORB"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Abstract: An amazingly innovative and rare NFT collection"
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
                                    "Abstract"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABSTRACT"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Absurd: An amazingly innovative and rare NFT collection"
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
                                    "Absurd"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABSURD"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Abuse: An amazingly innovative and rare NFT collection"
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
                                    "Abuse"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ABUSE"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Access: An amazingly innovative and rare NFT collection"
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
                                    "Access"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ACCESS"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Accident: An amazingly innovative and rare NFT collection"
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
                                    "Accident"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ACCIDENT"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Account: An amazingly innovative and rare NFT collection"
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
                                    "Account"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ACCOUNT"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Accuse: An amazingly innovative and rare NFT collection"
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
                                    "Accuse"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ACCUSE"
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
                    ),
                    NonFungibleLocalId("#2#") => Tuple(
                        Tuple(
                            "nf-number-2"
                        )
                    ),
                    NonFungibleLocalId("#3#") => Tuple(
                        Tuple(
                            "nf-number-3"
                        )
                    ),
                    NonFungibleLocalId("#4#") => Tuple(
                        Tuple(
                            "nf-number-4"
                        )
                    ),
                    NonFungibleLocalId("#5#") => Tuple(
                        Tuple(
                            "nf-number-5"
                        )
                    ),
                    NonFungibleLocalId("#6#") => Tuple(
                        Tuple(
                            "nf-number-6"
                        )
                    ),
                    NonFungibleLocalId("#7#") => Tuple(
                        Tuple(
                            "nf-number-7"
                        )
                    ),
                    NonFungibleLocalId("#8#") => Tuple(
                        Tuple(
                            "nf-number-8"
                        )
                    ),
                    NonFungibleLocalId("#9#") => Tuple(
                        Tuple(
                            "nf-number-9"
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
                                    "Achieve: An amazingly innovative and rare NFT collection"
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
                                    "Achieve"
                                )
                            ),
                            false
                        ),
                        "symbol" => Tuple(
                            Enum<1u8>(
                                Enum<0u8>(
                                    "ACHIEVE"
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

        """.trimIndent()
        val manifest = TransactionManifest.createMultipleNonFungibleTokens(
            addressOfOwner = AccountAddress.init(
                "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql"
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testFaucet() {
        val instructionsString = """
            CALL_METHOD
                Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
                "lock_fee"
                Decimal("5000")
            ;
            CALL_METHOD
                Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
                "free"
            ;
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "try_deposit_batch_or_abort"
                Expression("ENTIRE_WORKTOP")
                Enum<0u8>()
            ;

        """.trimIndent()
        val manifest = TransactionManifest.faucet(
            includeLockFeeInstruction = true,
            addressOfReceivingAccount = AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testMarkingAccountAsDAppDefinitionType() {
        val instructionsString = """
            SET_METADATA
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "account_type"
                Enum<0u8>(
                    "dapp definition"
                )
            ;

        """.trimIndent()
        val manifest = TransactionManifest.markingAccountAsDAppDefinitionType(
            accountAddress = AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testPerAssetTransfers() {
        val instructionsString = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("10")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("10")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "deposit"
                Bucket("bucket1")
            ;

        """.trimIndent()
        val manifest = TransactionManifest.perAssetTransfers(
            transfers = PerAssetTransfers(
                fromAccount = AccountAddress.init(
                    "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                ),
                fungibleResources = listOf(
                    PerAssetTransfersOfFungibleResource(
                        resource = PerAssetFungibleResource(
                            resourceAddress = ResourceAddress.init(
                                "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                            ),
                            divisibility = 18.toUByte()
                        ),
                        transfers = listOf(
                            PerAssetFungibleTransfer(
                                useTryDepositOrAbort = false,
                                amount = 10.toDecimal192(),
                                recipient = AssetsTransfersRecipient.ForeignAccount(
                                    value = AccountAddress("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                                )
                            )
                        )
                    )
                ),
                nonFungibleResources = emptyList()
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testPerRecipientTransfers() {
        val instructionsString = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("100")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("100")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "deposit"
                Bucket("bucket1")
            ;

        """.trimIndent()
        val manifest = TransactionManifest.perRecipientTransfers(
            transfers = PerRecipientAssetTransfers(
                addressOfSender = AccountAddress.init(
                    "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                ),
                transfers = listOf(
                    PerRecipientAssetTransfer(
                        recipient = AssetsTransfersRecipient.ForeignAccount(
                            value = AccountAddress("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                        ),
                        fungibles = listOf(
                            PerRecipientFungibleTransfer(
                                useTryDepositOrAbort = false,
                                amount = 100.toDecimal192(),
                                divisibility = 18.toUByte(),
                                resourceAddress = ResourceAddress.init(
                                    validatingAddress = "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                                )
                            )
                        ),
                        nonFungibles = emptyList()
                    )
                )
            )
        )

        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testSetOwnerKeyHashes() {
        val instructionsString = """
            SET_METADATA
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "owner_keys"
                Enum<143u8>(
                    Array<Enum>(
                        Enum<1u8>(
                            Bytes("e0133afae9724fe19392e605735479ba092eee902c041ccbe631243459")
                        )
                    )
                )
            ;

        """.trimIndent()

        val manifest = TransactionManifest.setOwnerKeysHashes(
            addressOfAccountOrPersona = AddressOfAccountOrPersona.Account(
                address = AccountAddress.init("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            ),
            ownerKeyHashes = listOf(
                PublicKeyHash.Ed25519(
                    Exactly29Bytes.init("e0133afae9724fe19392e605735479ba092eee902c041ccbe631243459".hexToBagOfBytes())
                )
            )
        )
        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testStakesClaim() {
        val instructionsString = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw_non_fungibles"
                Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
                Array<NonFungibleLocalId>(
                    NonFungibleLocalId("#0#")
                )
            ;
            TAKE_ALL_FROM_WORKTOP
                Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0")
                "claim_xrd"
                Bucket("bucket1")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("2")
                Bucket("bucket2")
            ;
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "deposit"
                Bucket("bucket2")
            ;

        """.trimIndent()

        val manifest = TransactionManifest.stakesClaim(
            accountAddress = AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            stakeClaims = listOf(
                StakeClaim(
                    validatorAddress = ValidatorAddress.init("validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"),
                    resourceAddress = NonFungibleResourceAddress.init(
                        validating = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
                    ),
                    ids = listOf(
                        NonFungibleLocalId.fromInt(value = 0.toULong())
                    ),
                    amount = 2.toDecimal192()
                )
            )
        )
        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testThirdPartyDeposits() {
        val instructionsString = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "set_default_deposit_rule"
                Enum<1u8>()
            ;

        """.trimIndent()

        val manifest = TransactionManifest.thirdPartyDepositUpdate(
            accountAddress = AccountAddress.init(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            from = ThirdPartyDeposits(
                depositRule = DepositRule.ACCEPT_ALL,
                assetsExceptionList = emptyList(),
                depositorsAllowList = emptyList()
            ),
            to = ThirdPartyDeposits(
                depositRule = DepositRule.DENY_ALL,
                assetsExceptionList = emptyList(),
                depositorsAllowList = emptyList()
            )
        )
        assertEquals(instructionsString, manifest.string)
    }

    @Test
    fun testModifyAddGuarantees() {
        val unmodified = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;

        """.trimIndent()
        val unmodifiedManifest = TransactionManifest.init(
            instructionsString = unmodified,
            networkId = NetworkId.MAINNET
        )

        val modifiedManifest = unmodifiedManifest.modifyAddGuarantees(
            guarantees = listOf(
                TransactionGuarantee(
                    amount = 1336.toDecimal192(),
                    instructionIndex = 2.toULong(),
                    resourceAddress = ResourceAddress.init(
                        validatingAddress = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                    ),
                    resourceDivisibility = 18.toUByte()
                )
            )
        )
        val expectedModified = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1336")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;

        """.trimIndent()

        assertEquals(expectedModified, modifiedManifest.string)
    }

    @Test
    fun testModifyLockFee() {
        val unmodified = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;

        """.trimIndent()
        val unmodifiedManifest = TransactionManifest.init(
            instructionsString = unmodified,
            networkId = NetworkId.MAINNET
        )

        val modifiedManifest = unmodifiedManifest.modifyLockFee(
            addressOfFeePayer = AccountAddress.init(
                validatingAddress = "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            fee = 0.61f.toDecimal192(),
        )
        val expectedModified = """
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;

        """.trimIndent()

        assertEquals(expectedModified, modifiedManifest.string)
    }
}