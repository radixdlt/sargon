package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class TransactionIntentHashTest: SampleTestable<TransactionIntentHash> {
    override val samples: List<Sample<TransactionIntentHash>>
        get() = listOf(TransactionIntentHash.sample)

    @Test
    fun testNetworkId() {
        assertEquals(
            NetworkId.MAINNET,
            TransactionIntentHash.sample().networkId
        )
        assertEquals(
            NetworkId.SIMULATOR,
            TransactionIntentHash.sample.other().networkId
        )
    }

    @Test
    fun testStringRoundtrip() {
        val txId = "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
        assertEquals(
            txId,
            TransactionIntentHash.init(txId).bech32EncodedTxId
        )
    }

    @Test
    fun testFormatted() {
        val txId = "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
        val formatted = "txid...zm3ltd"
        assertEquals(
            formatted,
            TransactionIntentHash.init(txId).formatted()
        )
        assertEquals(
            formatted,
            TransactionIntentHash.init(txId).formatted(format = AddressFormat.DEFAULT)
        )
        assertEquals(
            txId,
            TransactionIntentHash.init(txId).formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            txId,
            TransactionIntentHash.init(txId).formatted(format = AddressFormat.RAW)
        )
    }
}