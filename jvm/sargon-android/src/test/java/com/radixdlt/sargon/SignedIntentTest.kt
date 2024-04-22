package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class SignedIntentTest: SampleTestable<SignedIntent> {
    override val samples: List<Sample<SignedIntent>>
        get() = listOf(SignedIntent.sample)

    @Test
    fun testHash() {
        val s = "signedintent_sim1ul0kjuvd63sslhxy869zdk4k3vcdg9e9244xwpuck4dyndzx9wnqrhxy5d"
        Assertions.assertEquals(
            s,
            SignedIntent.sample().hash().bech32EncodedTxId
        )
    }
}