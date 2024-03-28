package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NotarySignatureTest: SampleTestable<NotarySignature> {
    override val samples: List<Sample<NotarySignature>>
        get() = listOf(NotarySignature.sample)

    @Test
    fun testSignatureRoundtrip() {
        assertEquals(
            NotarySignature.sample(),
            NotarySignature.init(NotarySignature.sample().signature)
        )
    }
}