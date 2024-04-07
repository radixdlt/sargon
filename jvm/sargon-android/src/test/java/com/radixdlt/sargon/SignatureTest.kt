package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SignatureTest: SampleTestable<Signature> {
    override val samples: List<Sample<Signature>>
        get() = listOf(Signature.sample)

    @Test
    fun testFromExactly64Bytes() {
        assertEquals(
            Signature.sample(),
            Signature.init(Signature.sample().bytes)
        )

        assertEquals(
            Signature.sample(),
            Signature.Ed25519.init(Signature.sample().bytes)
        )

        assertEquals(
            Signature.sample(),
            Signature.Ed25519.init(Signature.Ed25519.sample().bytes)
        )

        assertEquals(
            Signature.sample.other(),
            Signature.Secp256k1.init(Signature.Secp256k1.sample().bytes)
        )

        assertEquals(
            Signature.sample.other(),
            Signature.Secp256k1.init(Signature.Secp256k1.sample().bytes)
        )
    }

    @Test
    fun testFromBytes() {
        assertEquals(
            Signature.sample(),
            Signature.init(Signature.sample().bytes)
        )
    }

    @Test
    fun testString() {
        assertEquals(
            Signature.sample.other().string,
            Signature.init(Signature.sample.other().bytes).string
        )
    }

    @Test
    fun testAsGeneral() {
        val signatureEd25519 = Signature.Ed25519.sample()
        assertEquals(signatureEd25519, signatureEd25519.value.asGeneral())

        val signatureSecp256k1 = Signature.Secp256k1.sample()
        assertEquals(signatureSecp256k1, signatureSecp256k1.value.asGeneral())
    }
}