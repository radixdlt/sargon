package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class SignedTransactionIntentHashTest: SampleTestable<SignedTransactionIntentHash> {
    override val samples: List<Sample<SignedTransactionIntentHash>>
        get() = listOf(SignedTransactionIntentHash.sample)

    @Test
    fun testNetworkId() {
        Assertions.assertEquals(
            NetworkId.MAINNET,
            SignedTransactionIntentHash.sample().networkId
        )
        Assertions.assertEquals(
            NetworkId.SIMULATOR,
            SignedTransactionIntentHash.sample.other().networkId
        )
    }

    @Test
    fun testStringRoundtrip() {
        val s = "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl"
        Assertions.assertEquals(
            s,
            SignedTransactionIntentHash.init(s).bech32EncodedTxId
        )
    }

    @Test
    fun testFormatted() {
        val txId = "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl"
        val formatted = "sign...xsk6nl"
        Assertions.assertEquals(
            formatted,
            SignedTransactionIntentHash.init(txId).formatted()
        )
        Assertions.assertEquals(
            formatted,
            SignedTransactionIntentHash.init(txId).formatted(format = AddressFormat.DEFAULT)
        )
        Assertions.assertEquals(
            txId,
            SignedTransactionIntentHash.init(txId).formatted(format = AddressFormat.FULL)
        )
        Assertions.assertEquals(
            txId,
            SignedTransactionIntentHash.init(txId).formatted(format = AddressFormat.RAW)
        )
    }
}