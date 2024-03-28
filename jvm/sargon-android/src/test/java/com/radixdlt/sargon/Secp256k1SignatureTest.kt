package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Secp256k1SignatureTest: SampleTestable<Secp256k1Signature> {
    override val samples: List<Sample<Secp256k1Signature>>
        get() = listOf(Secp256k1Signature.sample)

    @Test
    fun testFromExactly64Bytes() {
        assertEquals(
            Secp256k1Signature.sample(),
            Secp256k1Signature.init(Secp256k1Signature.sample().bytes)
        )
    }

    @Test
    fun testFromBytes() {
        assertEquals(
            Secp256k1Signature.sample(),
            Secp256k1Signature.init(Secp256k1Signature.sample().bytes.bytes)
        )
    }

    @Test
    fun testString() {
        assertEquals(
            Secp256k1Signature.sample().string,
            Secp256k1Signature.init(Secp256k1Signature.sample().bytes).string
        )
    }
}