package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Ed25519SignatureTest: SampleTestable<Ed25519Signature> {
    override val samples: List<Sample<Ed25519Signature>>
        get() = listOf(Ed25519Signature.sample)

    @Test
    fun testFromExactly64Bytes() {
        assertEquals(
            Ed25519Signature.sample(),
            Ed25519Signature.init(Ed25519Signature.sample().bytes)
        )
    }

    @Test
    fun testFromBytes() {
        assertEquals(
            Ed25519Signature.sample(),
            Ed25519Signature.init(Ed25519Signature.sample().bytes.bytes)
        )
    }

    @Test
    fun testString() {
        assertEquals(
            Ed25519Signature.sample().string,
            Ed25519Signature.init(Ed25519Signature.sample().bytes).string
        )
    }
}