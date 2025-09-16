package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.accountLockerClaim
import com.radixdlt.sargon.extensions.blobs
import com.radixdlt.sargon.extensions.createFungibleToken
import com.radixdlt.sargon.extensions.createFungibleTokenWithMetadata
import com.radixdlt.sargon.extensions.createMultipleFungibleTokens
import com.radixdlt.sargon.extensions.createMultipleNonFungibleTokens
import com.radixdlt.sargon.extensions.createNonFungibleToken
import com.radixdlt.sargon.extensions.faucet
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.instructionsString
import com.radixdlt.sargon.extensions.involvedPoolAddresses
import com.radixdlt.sargon.extensions.involvedResourceAddresses
import com.radixdlt.sargon.extensions.manifestString
import com.radixdlt.sargon.extensions.markingAccountAsDAppDefinitionType
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.perAssetTransfers
import com.radixdlt.sargon.extensions.perRecipientTransfers
import com.radixdlt.sargon.extensions.setOwnerKeysHashes
import com.radixdlt.sargon.extensions.stakesClaim
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.summary
import com.radixdlt.sargon.extensions.thirdPartyDepositUpdate
import com.radixdlt.sargon.extensions.toDecimal192
import com.radixdlt.sargon.extensions.xrd
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import java.io.File
import java.util.regex.Pattern

class TransactionManifestTest : SampleTestable<TransactionManifest> {

    override val samples: List<Sample<TransactionManifest>>
        get() = listOf(TransactionManifest.sample)

    @Test
    fun test() {
        val instructionsString = TransactionManifest.sample().instructionsString

        val manifest =
            TransactionManifest.init(
                instructionsString = instructionsString,
                networkId = NetworkId.MAINNET,
                blobs = Blobs.sample()
            )

        assertEquals(instructionsString, manifest.instructionsString)
        assertEquals(NetworkId.MAINNET, manifest.networkId)
        assertEquals(Blobs.sample(), manifest.blobs)
    }

    @Test
    fun testCreateFungibleToken() {
        val manifest =
            TransactionManifest.createFungibleToken(
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY"))
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertEquals(1, occurrences("Stella"))
            assertEquals(1, occurrences("STAR"))
            assertEquals(
                1,
                occurrences(
                    "The brightest component in the Radix ecosystem."
                )
            )
        }
    }

    @Test
    fun testCreateNonFungibleToken() {
        val nftsPerCollection = 20
        var manifest =
            TransactionManifest.createNonFungibleToken(
                addressOfOwner =
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                ),
                nftsPerCollection = nftsPerCollection.toUByte()
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY"))
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertEquals(
                1,
                occurrences(
                    "An amazingly innovative and rare NFT collection"
                )
            )
            assertEquals(nftsPerCollection, occurrences("nf-number"))
        }

        // Can also skip specifying `nftsPerCollection`
        manifest =
            TransactionManifest.createNonFungibleToken(
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
        }
    }

    @Test
    fun testCreateFungibleTokenWithMetadata() {
        val manifest =
            TransactionManifest.createFungibleTokenWithMetadata(
                addressOfOwner =
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                ),
                initialSupply = 100.toDecimal192(),
                metadata =
                TokenDefinitionMetadata(
                    name = "Testname",
                    description =
                    "Test fungible",
                    symbol = "TST",
                    iconUrl =
                    "https://www.no-icon.com",
                    tags =
                    listOf(
                        "test"
                    )
                )
            )

        with(manifest.instructionsString) {
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertEquals(1, occurrences("Testname"))
            assertEquals(1, occurrences("Test fungible"))
            assertEquals(1, occurrences("TST"))
        }
    }

    @Test
    fun testCreateMultipleFungibleTokens() {
        val manifest =
            TransactionManifest.createMultipleFungibleTokens(
                addressOfOwner =
                AccountAddress.init(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY"))
            assertTrue(
                contains(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                )
            )
            assertEquals(10, occurrences("symbol"))
        }
    }

    @Test
    fun testCreateMultipleFungibleTokens_specify_count() {
        val count: UByte = 3u
        val manifest =
            TransactionManifest.createMultipleFungibleTokens(
                addressOfOwner =
                AccountAddress.init(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                ),
                count = count
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY"))
            assertTrue(
                contains(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                )
            )
            assertEquals(count.toInt(), occurrences("symbol"))
        }
    }

    @Test
    fun testCreateMultipleNonFungibleTokens() {
        val collections = 15
        val nftsPerCollection = 10

        var manifest =
            TransactionManifest.createMultipleNonFungibleTokens(
                addressOfOwner =
                AccountAddress.init(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                ),
                collectionCount = collections.toUByte(),
                nftsPerCollection = nftsPerCollection.toUByte()
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY"))
            assertTrue(
                contains(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                )
            )
            assertEquals(
                collections,
                occurrences(
                    "An amazingly innovative and rare NFT collection"
                )
            )
            assertEquals(collections * nftsPerCollection, occurrences("nf-number"))
        }

        // Can also skip specifying `collectionCount` / `nftsPerCollection`
        manifest =
            TransactionManifest.createMultipleNonFungibleTokens(
                addressOfOwner =
                AccountAddress.init(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(
                contains(
                    "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
                )
            )
        }
    }

    @Test
    fun testFaucetWithLockFee() {
        val manifest =
            TransactionManifest.faucet(
                includeLockFeeInstruction = true,
                addressOfReceivingAccount =
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CALL_METHOD"))
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertTrue(contains("lock_fee"))
        }
    }

    @Test
    fun testFaucetWithoutLockFee() {
        val manifest =
            TransactionManifest.faucet(
                includeLockFeeInstruction = false,
                addressOfReceivingAccount =
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(contains("CALL_METHOD"))
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertFalse(contains("lock_fee"))
        }
    }

    @Test
    fun testMarkingAccountAsDAppDefinitionType() {
        val manifest =
            TransactionManifest.markingAccountAsDAppDefinitionType(
                accountAddress =
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )

        with(manifest.instructionsString) {
            assertTrue(contains("SET_METADATA"))
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertTrue(contains("dapp definition"))
        }
    }

    @Test
    fun testPerAssetTransfers() {
        val transfers = PerAssetTransfers.sample()
        val manifest = TransactionManifest.perAssetTransfers(transfers = transfers)

        with(manifest.instructionsString) {
            assertTrue(contains(transfers.fromAccount.string))
            transfers.fungibleResources.forEach { perAssetTransfer ->
                assertTrue(
                    contains(
                        perAssetTransfer.resource
                            .resourceAddress
                            .string
                    )
                )
                perAssetTransfer.transfers.forEach {
                    when (val recipient = it.recipient) {
                        is TransferRecipient.AddressOfExternalAccount ->
                            assertTrue(
                                contains(
                                    recipient.value
                                        .string
                                )
                            )
                        is TransferRecipient.ProfileAccount ->
                            assertTrue(
                                contains(
                                    recipient.value.address
                                        .string
                                )
                            )
                        is TransferRecipient.RnsDomain ->
                            assertTrue(
                                contains(
                                    recipient.value.receiver
                                        .string
                                )
                            )
                    }
                }
            }
        }
    }

    @Test
    fun testPerRecipientTransfers() {
        val manifest =
            TransactionManifest.perRecipientTransfers(
                transfers =
                PerRecipientAssetTransfers(
                    addressOfSender =
                    AccountAddress.init(
                        "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                    ),
                    transfers =
                    listOf(
                        PerRecipientAssetTransfer(
                            recipient =
                            TransferRecipient
                                .AddressOfExternalAccount(
                                    value =
                                    AccountAddress(
                                        "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
                                    )
                                ),
                            fungibles =
                            listOf(
                                PerRecipientFungibleTransfer(
                                    useTryDepositOrAbort =
                                    false,
                                    amount =
                                    100.toDecimal192(),
                                    divisibility =
                                    18.toUByte(),
                                    resourceAddress =
                                    ResourceAddress.init(
                                        validatingAddress =
                                        "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                                    )
                                )
                            ),
                            nonFungibles =
                            emptyList()
                        )
                    )
                )
            )

        with(manifest.instructionsString) {
            assertEquals(
                1,
                occurrences(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertEquals(
                2,
                occurrences(
                    "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                )
            )
            assertEquals(
                1,
                occurrences(
                    "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
                )
            )
        }
    }

    @Test
    fun testSetOwnerKeyHashes() {
        val manifest =
            TransactionManifest.setOwnerKeysHashes(
                addressOfAccountOrPersona =
                AddressOfAccountOrPersona.Account(
                    AccountAddress.init(
                        "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                    )
                ),
                ownerKeyHashes =
                listOf(
                    PublicKeyHash.Ed25519(
                        Exactly29Bytes.init(
                            "e0133afae9724fe19392e605735479ba092eee902c041ccbe631243459".hexToBagOfBytes()
                        )
                    )
                )
            )
        with(manifest.instructionsString) {
            assertTrue(contains("SET_METADATA"))
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertTrue(contains("owner_keys"))
        }
    }

    @Test
    fun testStakesClaim() {
        val accountAddress =
            AccountAddress.init(
                "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
            )
        val manifest =
            TransactionManifest.stakesClaim(
                accountAddress = accountAddress,
                stakeClaims = StakeClaim.sample.all
            )
        with(manifest.instructionsString) {
            assertEquals(2, occurrences("validator_"))
            assertEquals(
                2,
                occurrences(
                    ResourceAddress.xrd(
                        accountAddress.networkId
                    )
                        .string
                )
            )
            assertTrue(contains(accountAddress.string))
        }
    }

    @Test
    fun testThirdPartyDeposits() {
        val manifest =
            TransactionManifest.thirdPartyDepositUpdate(
                accountAddress =
                AccountAddress.init(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                ),
                from = ThirdPartyDeposits.sample(),
                to = ThirdPartyDeposits.sample.other()
            )
        with(manifest.instructionsString) {
            assertTrue(
                contains(
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                )
            )
            assertEquals(3, occurrences(";"))
        }
    }

    @Test
    fun test_manifest_string() {
        val manifest = TransactionManifest.sample()
        assertTrue(manifest.manifestString.contains("CALL_METHOD"))
    }

    @Test
    fun test_involved_resource_addresses() {
        assertEquals(
            listOf(ResourceAddress.sampleMainnet.xrd),
            TransactionManifest.sample().involvedResourceAddresses
        )
    }

    @Test
    fun test_involved_pool_addresses() {
        assertEquals(
            emptyList<PoolAddress>(),
            TransactionManifest.sample().involvedPoolAddresses
        )
    }

    @Test
    fun test_summary() {
        val summary = TransactionManifest.sample().summary
        assertEquals(
            listOf(AccountAddress.sampleMainnet()),
            summary.addressesOfAccountsWithdrawnFrom
        )
    }

    @Test
    fun testAccountLockerClaim() {
        val expectedManifest = TransactionManifest.init(
            instructionsString = openFileInManifests("account_locker_claim", "rtm").readText(),
            networkId = NetworkId.MAINNET
        )
        var actualManifest = TransactionManifest.accountLockerClaim(
            lockerAddress = LockerAddress.init("locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz"),
            claimant = AccountAddress.init("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"),
            claimableResources = listOf(
                AccountLockerClaimableResource.Fungible(
                    resourceAddress = ResourceAddress.init("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"),
                    amount = 123.toDecimal192()
                ),
                AccountLockerClaimableResource.NonFungible(
                    resourceAddress = ResourceAddress.init("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"),
                    numberOfItems = 10.toULong()
                ),
                AccountLockerClaimableResource.Fungible(
                    resourceAddress = ResourceAddress.init("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"),
                    amount = 1500.toDecimal192()
                ),
                AccountLockerClaimableResource.NonFungible(
                    resourceAddress = ResourceAddress.init("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"),
                    numberOfItems = 1.toULong()
                )
            )
        )

        assertEquals(expectedManifest.instructionsString, actualManifest.instructionsString)

        with(actualManifest.instructionsString) {
            assertTrue(contains("claim"))
            assertTrue(contains("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"))
        }

        actualManifest = TransactionManifest.accountLockerClaim(
            lockerAddress = LockerAddress.init("locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz"),
            claimant = AccountAddress.init("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"),
            claimableResources = emptyList()
        )

        with(actualManifest.instructionsString) {
            assertTrue(isEmpty())
        }
    }

    private fun String.occurrences(substring: String): Int {
        val matcher = Pattern.compile(substring).matcher(this)
        var counter = 0
        while (matcher.find()) {
            counter++
        }
        return counter
    }

    private fun manifest(name: String) =
        TransactionManifest.init(
            instructionsString = openFileInTxModels(name, "rtm").readText(),
            networkId = NetworkId.STOKENET
        )

    private fun engineToolkitReceipt(name: String): String = openFileInTxModels(name, "json").readText()

    private fun openFileInCrate(crate: String, name: String, extension: String) =
        File("../../" + "fixtures/transaction/$name.$extension")

    private fun openFileInTxModels(name: String, extension: String) = openFileInCrate("transaction/models", name, extension)
    private fun openFileInManifests(name: String, extension: String) = openFileInCrate("transaction/manifests", name, extension)
}
