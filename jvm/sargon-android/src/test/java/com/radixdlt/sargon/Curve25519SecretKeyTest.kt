package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Curve25519SecretKey
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Test

class Curve25519SecretKeyTest {

    @Test
    fun testRandomness() {
        val privateKeysCount = 100

        val privateKeys = List(privateKeysCount) {
            Curve25519SecretKey.secureRandom()
        }

        assertEquals(
            privateKeys.toSet().size,
            privateKeysCount
        )
    }

    @Test
    fun testEquality() {
        val thisNotary = Curve25519SecretKey(Exactly32Bytes.sample())
        val otherNotary = Curve25519SecretKey(Exactly32Bytes.sample())

        assertEquals(thisNotary, otherNotary)
        assertEquals(thisNotary, thisNotary)
        assertFalse(thisNotary.equals(null))
    }

    @Test
    fun testNotarize() {
        val sut = Curve25519SecretKey(Exactly32Bytes.sample())
        val result = sut.notarize(SignedIntentHash.sample())

        assertEquals(
            "1a30347a04bc5d746b35a568330ba69c9b6ac60ef72d0a28cb63e25680e64908557d85a0e864c423ce782b5f43da3002c301045c6385b40cb013374045392404",
            result.signature.string
        )
    }

    @Test
    fun testSign() {
        val sut = Curve25519SecretKey(Exactly32Bytes.sample())
        val result = sut.sign(Hash.sample())

        assertEquals(
            "1a30347a04bc5d746b35a568330ba69c9b6ac60ef72d0a28cb63e25680e64908557d85a0e864c423ce782b5f43da3002c301045c6385b40cb013374045392404",
            result.bytes.hex
        )
    }

    @Test
    fun testGetPublicKey() {
        val sut = Curve25519SecretKey(Exactly32Bytes.sample())

        assertEquals(
            "3b321b74bdcb169f7260c60592bbb63d9b4d629424a0c58aff4640a75f0a2b06",
            sut.toPublicKey().hex
        )
    }

}