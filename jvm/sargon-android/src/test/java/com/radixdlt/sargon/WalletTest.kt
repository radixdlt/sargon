package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.invoke
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import kotlin.random.Random

class WalletTest {

    @Test
    fun test() {
        println("🚀 Test Wallet in Kotlin start")

        val storage = EphemeralKeystore
        assertTrue(storage.isEmpty())

        println("🔮 GENERATING NEW WALLET")
        val wallet = Wallet.with(entropy = ByteArray(32) { 0xFF.toByte() }, secureStorage = storage)

        assertTrue(
            storage.contains(
                value =
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"
            )
        )
        println("✨ SUCCESSFULLY GENERATED NEW WALLET ✅")

        println("🔮 Creating first account on mainnet")
        val initialNameOfFirstAccount = "Alice"
        // Not created any account yet...
        assertFalse(storage.contains(value = initialNameOfFirstAccount))
        assertTrue(wallet.profile().networks().isEmpty())
        var main0 = wallet.createAndSaveNewAccount(
            networkId = NetworkId.MAINNET,
            name = DisplayName.init(validating = initialNameOfFirstAccount)
        )
        assertEquals(NetworkId.MAINNET, main0.networkId)
        assertEquals(1, wallet.profile().networks.size)
        assertEquals(1, wallet.profile().networks[0].accounts.size)
        assertEquals(
            initialNameOfFirstAccount,
            wallet.profile().networks[0].accounts[0].displayName.value
        )
        assertTrue(storage.contains(value = initialNameOfFirstAccount))
        println("✨ Successfully created first account ✅")

        println("🔮 Update account using `update_account`")
        var updatedNameOfFirstAccount = "Stella"
        main0.displayName = DisplayName.init(validating = updatedNameOfFirstAccount)
        main0.appearanceId = AppearanceId.sample.other()
        val main0Updated = wallet.updateAccount(to = main0)
        assertEquals(main0, main0Updated)
        assertEquals(
            updatedNameOfFirstAccount,
            wallet.profile().networks[0].accounts[0].displayName.value
        )
        assertEquals(
            AppearanceId.sample.other(),
            wallet.profile().networks[0].accounts[0].appearanceId
        )
        assertTrue(storage.contains(value = updatedNameOfFirstAccount))
        println("✨ Successfully updated first account using `update_account` ✅")

        println("🔮 Renaming account using changeNameOfAccount")
        updatedNameOfFirstAccount = "Satoshi"
        main0 = wallet.changeNameOfAccount(
            address = main0.address,
            to = DisplayName.init(validating = updatedNameOfFirstAccount)
        )
        assertEquals(
            updatedNameOfFirstAccount,
            wallet.profile().networks[0].accounts[0].displayName.value
        )
        assertTrue(storage.contains(value = updatedNameOfFirstAccount))
        println("✨ Successfully renamed first account using changeNameOfAccount ✅")

        println("🔮 Creating second mainnet account")
        val main1 = wallet.createAndSaveNewAccount(
            networkId = NetworkId.MAINNET,
            name = DisplayName.init(validating = "Bob")
        )
        assertNotEquals(main0.address, main1.address)
        assertEquals(main0.networkId, main1.networkId)
        assertEquals(1, wallet.profile().networks.size)
        assertEquals(listOf(main0, main1), wallet.profile().networks[0].accounts())

        println("🔮 Creating first testnet account")
        val testnetAccountName = "Hello Radix Account!"
        val test0 = wallet.createAndSaveNewAccount(
            networkId = NetworkId.STOKENET,
            name = DisplayName.init(validating = testnetAccountName)
        )
        assertEquals(2, wallet.profile().networks.size)
        assertEquals(listOf(test0), wallet.profile().networks[1].accounts())
        assertEquals(testnetAccountName, wallet.profile().networks[1].accounts[0].displayName.value)
        assertEquals(NetworkId.STOKENET, wallet.profile().networks[1].accounts[0].networkId)
        assertTrue(storage.contains(value = testnetAccountName))
        println("✨ Successfully created first testnet account ✅")

        println("✅ Test Wallet in Kotlin completed ")
    }

    val Wallet.Companion.defaultPhoneName: String
        get() = "Android Phone"

    fun Wallet.Companion.with(
        entropy: ByteArray = ByteArray(32).apply { Random.nextBytes(this) },
        phoneName: String = Wallet.Companion.defaultPhoneName,
        secureStorage: SecureStorageDriver
    ): Wallet {
        return Wallet.byCreatingNewProfileAndSecretsWithEntropy(
            entropy = NonEmptyMax32Bytes(bagOfBytes = entropy.toBagOfBytes()),
            walletClientModel = WalletClientModel.ANDROID,
            walletClientName = phoneName,
            secureStorage = secureStorage
        )
    }

}