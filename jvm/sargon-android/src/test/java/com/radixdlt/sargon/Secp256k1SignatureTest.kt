package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Secp256k1SignatureTest: SampleTestable<Signature.Secp256k1> {
    override val samples: List<Sample<Signature.Secp256k1>>
        get() = listOf(Signature.Secp256k1.sample)

    @Test
    fun testFromExactly64Bytes() {
        assertEquals(
            Signature.Secp256k1.sample(),
            Signature.Secp256k1.init(Signature.Secp256k1.sample().value.bytes)
        )
    }

    @Test
    fun testFromBytes() {
        assertEquals(
            Signature.Secp256k1.sample(),
            Signature.Secp256k1.init(Signature.Secp256k1.sample().bytes)
        )
    }

    @Test
    fun testString() {
        assertEquals(
            Signature.Secp256k1.sample().string,
            Signature.Secp256k1.init(Signature.Secp256k1.sample().bytes).string
        )
    }
}