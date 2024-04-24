package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.derivePublicKey
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isValidSignature
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.extensions.sign
import com.radixdlt.sargon.extensions.signature
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
            passphrase = "super secret"
        )

        assertEquals(
            mnemonicWithPassphrase,
            MnemonicWithPassphrase.fromJson(mnemonicWithPassphrase.toJson())
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

    @Test
    fun testSignIsValid() {
        val sut = MnemonicWithPassphrase.sample()
        val derivationPath = DerivationPath.sample()
        val message = Hash.sample()

        val publicKey = sut.derivePublicKey(path = derivationPath)
        val signatureWithPublicKey = sut.sign(message, derivationPath)
        assertTrue(
            publicKey.isValidSignature(signatureWithPublicKey.signature, message)
        )
    }

    @Test
    fun testDerivePublicKeys() {
        assertEquals(
            HierarchicalDeterministicPublicKey.sample(),
            MnemonicWithPassphrase.sample().derivePublicKey(DerivationPath.sample())
        )
    }

}