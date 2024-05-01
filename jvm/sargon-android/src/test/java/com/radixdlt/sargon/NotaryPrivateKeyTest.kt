package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.NotaryPrivateKey
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Test

class NotaryPrivateKeyTest {

    @Test
    fun testRandomness() {
        val privateKeysCount = 100

        val privateKeys = List(privateKeysCount) {
            NotaryPrivateKey.secureRandom()
        }

        assertEquals(
            privateKeys.toSet().size,
            privateKeysCount
        )
    }

    @Test
    fun testEquality() {
        val thisNotary = NotaryPrivateKey(Entropy32Bytes.sample())
        val otherNotary = NotaryPrivateKey(Entropy32Bytes.sample())

        assertEquals(thisNotary, otherNotary)
        assertEquals(thisNotary, thisNotary)
        assertFalse(thisNotary.equals(null))
    }

    @Test
    fun testNotarize() {
        val sut = NotaryPrivateKey(Entropy32Bytes.sample())
        val result = sut.notarize(SignedIntentHash.sample())

        assertEquals(
            "08c6129fa6938a31e38dfe94effdce8f1a4021e22cf62344830d83dc45f32de0e3d112794c369450e62d245a17b18835f40c639033fbb4b1f975ad0ad71dbf0a",
            result.signature.string
        )
    }

    @Test
    fun testGetPublicKey() {
        val sut = NotaryPrivateKey(Entropy32Bytes.sample())

        Assertions.assertEquals(
            "248acbdbaf9e050196de704bea2d68770e519150d103b587dae2d9cad53dd930",
            sut.toPublicKey().hex
        )
    }

}