package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.deserializeFromString
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.serializedString
import com.radixdlt.sargon.extensions.validate
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class MnemonicWithPassphraseTest {

    @Test
    fun testJsonRoundtrip() {
        val mnemonicWithPassphrase = MnemonicWithPassphrase(
            mnemonic = Mnemonic.init("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"),
            passphrase = "passphrase"
        )

        assertEquals(
            mnemonicWithPassphrase,
            MnemonicWithPassphrase.deserializeFromString(mnemonicWithPassphrase.serializedString())
        )
    }

    @Test
    fun testHDPublicKeyValidation() {
        assertTrue(
            MnemonicWithPassphrase.sample()
                .validate(listOf(HierarchicalDeterministicPublicKey.sample())),
        )
        assertFalse(
            MnemonicWithPassphrase.sample.other()
                .validate(listOf(HierarchicalDeterministicPublicKey.sample())),
        )
    }

}