package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.decompile
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class CompiledTransactionIntentTest: SampleTestable<CompiledTransactionIntent> {
    override val samples: List<Sample<CompiledTransactionIntent>>
        get() = listOf(CompiledTransactionIntent.sample)

    @Test
    fun testDecompile() {
        val expectedTransaction = TransactionIntent.sample()
        assertEquals(
            expectedTransaction,
            expectedTransaction.compile().decompile()
        )
    }
}