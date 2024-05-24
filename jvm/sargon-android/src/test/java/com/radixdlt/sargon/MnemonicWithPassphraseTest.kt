package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.derivePublicKey
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isValidSignature
import com.radixdlt.sargon.extensions.phrase
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.extensions.sign
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.validate
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

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
    fun testInitFromJustPhrase() {
        val phrase = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
        assertEquals(phrase, MnemonicWithPassphrase.init(phrase = phrase).mnemonic.phrase)
    }

    @Test
    fun testFromAndroidJson() {
        val androidJsonWithoutPassphrase =
            """{"mnemonic":"remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track","bip39Passphrase":""}""".trimIndent()
        assertEquals(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init(phrase = "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = ""
            ),
            MnemonicWithPassphrase.fromJson(androidJsonWithoutPassphrase)
        )

        val androidJsonWithoutPassphrasePrettyPrinted = """{
              "mnemonic": "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track",
              "bip39Passphrase": ""
            }""".trimIndent()
        assertEquals(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init(phrase = "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = ""
            ),
            MnemonicWithPassphrase.fromJson(androidJsonWithoutPassphrasePrettyPrinted)
        )

        val androidJsonWithPassphrase =
            """{"mnemonic":"remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track","bip39Passphrase":"super secret"}""".trimIndent()
        assertEquals(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init(phrase = "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = "super secret"
            ),
            MnemonicWithPassphrase.fromJson(androidJsonWithPassphrase)
        )
    }

    @Test
    fun testFromInvalidJson() {
        val invalidJson = "{}"
        assertThrows<CommonException.FailedToDeserializeJsonToValue> {
            MnemonicWithPassphrase.fromJson(invalidJson)
        }


        val iOSJsonLike = mnemonicWithPassphraseToJsonBytes(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init("remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = "super secret"
            )
        ).string

        assertThrows<CommonException.FailedToDeserializeJsonToValue> {
            MnemonicWithPassphrase.fromJson(iOSJsonLike)
        }
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