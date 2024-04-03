package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class IntentHashTest: SampleTestable<IntentHash> {
    override val samples: List<Sample<IntentHash>>
        get() = listOf(IntentHash.sample)

    @Test
    fun testNetworkId() {
        assertEquals(
            NetworkId.MAINNET,
            IntentHash.sample().networkId
        )
        assertEquals(
            NetworkId.SIMULATOR,
            IntentHash.sample.other().networkId
        )
    }

    @Test
    fun testStringRoundtrip() {
        val txId = "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
        assertEquals(
            txId,
            IntentHash.init(txId).bech32EncodedTxId
        )
    }

    @Test
    fun testFormatted() {
        val txId = "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
        val formatted = "txid...zm3ltd"
        assertEquals(
            formatted,
            IntentHash.init(txId).formatted()
        )
        assertEquals(
            formatted,
            IntentHash.init(txId).formatted(format = AddressFormat.DEFAULT)
        )
        assertEquals(
            txId,
            IntentHash.init(txId).formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            txId,
            IntentHash.init(txId).formatted(format = AddressFormat.RAW)
        )
    }
}