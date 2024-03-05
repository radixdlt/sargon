package com.radixdlt.sargon.android

import com.radixdlt.sargon.AppearanceId
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.SecureStorage
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.Wallet
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.newDisplayName
import com.radixdlt.sargon.sample.sample
import com.radixdlt.sargon.secureStorageKeyIdentifier
import org.junit.Test
import kotlin.random.Random

class WalletTest {

    @Test
    fun test() {
        println("🚀 Test Wallet in Kotlin start")

        val storage = EphemeralKeystore() // Cannot use Object in kotlin script
        assert(storage.isEmpty())

        println("🔮 GENERATING NEW WALLET")
        val wallet = Wallet.with(entropy = ByteArray(32) { 0xFF.toByte() }, secureStorage = storage)

        assert(
                storage.contains(
                        value =
                                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"
                )
        )
        println("✨ SUCCESSFULLY GENERATED NEW WALLET ✅")

        println("🔮 Creating first account on mainnet")
        val initialNameOfFirstAccount = "Alice"
        // Not created any account yet...
        assert(!storage.contains(value = initialNameOfFirstAccount))
        assert(wallet.profile().networks.isEmpty())
        var main0 =
                wallet.createAndSaveNewAccount(
                        networkId = NetworkId.MAINNET,
                        name = DisplayName.from(value = initialNameOfFirstAccount)
                )
        assert(main0.networkId == NetworkId.MAINNET)
        assert(wallet.profile().networks.size == 1)
        assert(wallet.profile().networks[0].accounts.size == 1)
        assert(
                wallet.profile().networks[0].accounts[0].displayName.value ==
                        initialNameOfFirstAccount
        )
        assert(storage.contains(value = initialNameOfFirstAccount))
        print("✨ Successfully created first account ✅")

        print("🔮 Update account using `update_account`")
        var updatedNameOfFirstAccount = "Stella"
        main0.displayName = DisplayName.from(value = updatedNameOfFirstAccount)
        main0.appearanceId = AppearanceId.sample.other()
        val main0Updated = wallet.updateAccount(to = main0)
        assert(main0Updated == main0)
        assert(
                wallet.profile().networks[0].accounts[0].displayName.value ==
                        updatedNameOfFirstAccount
        )
        assert(wallet.profile().networks[0].accounts[0].appearanceId == AppearanceId.sample.other())
        assert(storage.contains(value = updatedNameOfFirstAccount))
        print("✨ Successfully updated first account using `update_account` ✅")

        print("🔮 Renaming account using changeNameOfAccount")
        updatedNameOfFirstAccount = "Satoshi"
        main0 =
                wallet.changeNameOfAccount(
                        address = main0.address,
                        to = DisplayName.from(value = updatedNameOfFirstAccount)
                )
        assert(
                wallet.profile().networks[0].accounts[0].displayName.value ==
                        updatedNameOfFirstAccount
        )
        assert(storage.contains(value = updatedNameOfFirstAccount))
        print("✨ Successfully renamed first account using changeNameOfAccount ✅")

        print("🔮 Creating second mainnet account")
        val main1 =
                wallet.createAndSaveNewAccount(
                        networkId = NetworkId.MAINNET,
                        name = DisplayName.from(value = "Bob")
                )
        assert(main0.address != main1.address)
        assert(main0.networkId == main1.networkId)
        assert(wallet.profile().networks.size == 1)
        assert(wallet.profile().networks[0].accounts == listOf(main0, main1))

        print("🔮 Creating first testnet account")
        val testnetAccountName = "Hello Radix Account!"
        val test0 =
                wallet.createAndSaveNewAccount(
                        networkId = NetworkId.STOKENET,
                        name = DisplayName.from(value = testnetAccountName)
                )
        assert(wallet.profile().networks.size == 2)
        assert(wallet.profile().networks[1].accounts == listOf(test0))
        assert(wallet.profile().networks[1].accounts[0].displayName.value == testnetAccountName)
        assert(wallet.profile().networks[1].accounts[0].networkId == NetworkId.STOKENET)
        assert(storage.contains(value = testnetAccountName))
        println("✨ Successfully created first testnet account ✅")

        println("✅ Test Wallet in Kotlin completed ")
    }

    fun DisplayName.Companion.from(value: String): DisplayName {
        return newDisplayName(name = value)
    }

    val Wallet.Companion.defaultPhoneName: String
        get() = "Android Phone"

    fun Wallet.Companion.with(
            entropy: ByteArray = ByteArray(32).apply { Random.nextBytes(this) },
            phoneName: String = Wallet.Companion.defaultPhoneName,
            secureStorage: SecureStorage
    ): Wallet {
        return Wallet.byCreatingNewProfileAndSecretsWithEntropy(
                entropy = entropy,
                walletClientModel = WalletClientModel.ANDROID,
                walletClientName = phoneName,
                secureStorage = secureStorage
        )
    }

    class EphemeralKeystore : SecureStorage {
        private val storage: MutableMap<String, ByteArray> = mutableMapOf()

        override fun loadData(key: SecureStorageKey): ByteArray? = storage[key.identifier]

        override fun saveData(key: SecureStorageKey, data: ByteArray) {
            storage[key.identifier] = data
        }

        override fun deleteDataForKey(key: SecureStorageKey) {
            storage.remove(key = key.identifier)
        }

        fun isEmpty() = storage.isEmpty()

        fun contains(value: String): Boolean {
            return storage.any { entry -> entry.value.decodeToString().contains(value) }
        }

        private val SecureStorageKey.identifier: String
            get() = secureStorageKeyIdentifier(this)
    }
}
