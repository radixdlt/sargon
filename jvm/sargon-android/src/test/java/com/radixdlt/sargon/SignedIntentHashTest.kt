package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class SignedIntentHashTest: SampleTestable<SignedIntentHash> {
    override val samples: List<Sample<SignedIntentHash>>
        get() = listOf(SignedIntentHash.sample)

    @Test
    fun testNetworkId() {
        Assertions.assertEquals(
            NetworkId.MAINNET,
            SignedIntentHash.sample().networkId
        )
        Assertions.assertEquals(
            NetworkId.SIMULATOR,
            SignedIntentHash.sample.other().networkId
        )
    }

    @Test
    fun testStringRoundtrip() {
        val s = "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl"
        Assertions.assertEquals(
            s,
            SignedIntentHash.init(s).bech32EncodedTxId
        )
    }
}