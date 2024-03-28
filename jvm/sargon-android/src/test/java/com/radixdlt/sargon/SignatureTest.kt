package com.radixdlt.sargon

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
            Signature.Ed25519.init(Ed25519Signature.sample().bytes)
        )

        assertEquals(
            Signature.sample(),
            Signature.Ed25519.init(Ed25519Signature.sample().bytes.bytes)
        )

        assertEquals(
            Signature.sample.other(),
            Signature.Secp256k1.init(Secp256k1Signature.sample().bytes)
        )

        assertEquals(
            Signature.sample.other(),
            Signature.Secp256k1.init(Secp256k1Signature.sample().bytes.bytes)
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
}