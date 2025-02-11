package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.default
import com.radixdlt.sargon.extensions.derivePublicKey
import com.radixdlt.sargon.extensions.factorSourceId
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.initFromLocal
import com.radixdlt.sargon.extensions.isLegacy
import com.radixdlt.sargon.extensions.isUnsecuredLedgerControlled
import com.radixdlt.sargon.extensions.unsecuredControllingFactorInstance
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AccountTest : SampleTestable<Account> {
    override val samples: List<Sample<Account>>
        get() = listOf(Account.sampleMainnet, Account.sampleStokenet)

    @Test
    fun testIsLegacyOlympia() {
        assertEquals(
            Account.sampleMainnet().isLegacy,
            false
        )
    }

    @Test
    fun testUnsecuredLedgerControlledAccount() {
        assert(unsecuredLedgerControlledAccountWithFactorInstance().first.isUnsecuredLedgerControlled)
    }

    @Test
    fun testDeviceAccountIsUnsecuredLedgerControlledReturnsFalse() {
        assert(!Account.sampleMainnet().isUnsecuredLedgerControlled)
    }

    @Test
    fun testSecurifiedAccountIsUnsecuredLedgerControlledReturnsFalse() {
        val securifiedAccount = Account(
            networkId = NetworkId.MAINNET,
            address = AccountAddress.sampleMainnet(),
            displayName = DisplayName.init("Securified"),
            securityState = EntitySecurityState.Securified(
                SecuredEntityControl(
                    veci = HierarchicalDeterministicFactorInstance.sample(),
                    accessControllerAddress = AccessControllerAddress.sampleMainnet(),
                    securityStructure = SecurityStructureOfFactorInstances.sample(),
                    provisionalSecurifiedConfig = null
                )
            ),
            appearanceId = AppearanceId.sample(),
            flags = emptyList(),
            onLedgerSettings = OnLedgerSettings.default()
        )

        assert(!securifiedAccount.isUnsecuredLedgerControlled)
    }

    @Test
    fun testUnsecuredControllingFactorInstanceForAccount() {
        val (account, hdPublicKey) = unsecuredLedgerControlledAccountWithFactorInstance()
        assertEquals(
            account.unsecuredControllingFactorInstance,
            hdPublicKey
        )
    }

    private fun unsecuredLedgerControlledAccountWithFactorInstance():
            Pair<Account, HierarchicalDeterministicFactorInstance> {
        val mnemonic = MnemonicWithPassphrase.sample()
        val factorSourceId = mnemonic.factorSourceId(
            kind = FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET
        )
        val derivationPath = AccountPath.init(
            networkId = NetworkId.MAINNET,
            keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
            index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
        ).asGeneral()
        val hdPublicKey = mnemonic.derivePublicKey(derivationPath)

        val factorInstance = HierarchicalDeterministicFactorInstance(
            factorSourceId = factorSourceId,
            publicKey = hdPublicKey
        )
        return Account(
            networkId = NetworkId.MAINNET,
            address = AccountAddress.init(
                publicKey = hdPublicKey.publicKey,
                networkId = NetworkId.MAINNET
            ),
            displayName = DisplayName.init("Ledger Controlled"),
            securityState = EntitySecurityState.Unsecured(
                value = UnsecuredEntityControl(
                    transactionSigning = factorInstance,
                    provisionalSecurifiedConfig = null
                )
            ),
            appearanceId = AppearanceId.sample(),
            flags = emptyList(),
            onLedgerSettings = OnLedgerSettings.default()
        ) to factorInstance
    }
}