package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Ed25519SignatureTest: SampleTestable<Signature.Ed25519> {
    override val samples: List<Sample<Signature.Ed25519>>
        get() = listOf(Signature.Ed25519.sample)

    @Test
    fun testFromExactly64Bytes() {
        assertEquals(
            Signature.Ed25519.sample(),
            Signature.Ed25519.init(Signature.Ed25519.sample().value.bytes)
        )
    }

    @Test
    fun testFromBytes() {
        assertEquals(
            Signature.Ed25519.sample(),
            Signature.Ed25519.init(Signature.Ed25519.sample().bytes)
        )
    }

    @Test
    fun testString() {
        assertEquals(
            Signature.Ed25519.sample().string,
            Signature.Ed25519.init(Signature.Ed25519.sample().bytes).string
        )
    }
}