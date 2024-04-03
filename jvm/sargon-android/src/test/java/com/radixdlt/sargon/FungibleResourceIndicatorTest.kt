package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.amount
import com.radixdlt.sargon.extensions.toDecimal192
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class FungibleResourceIndicatorTest: SampleTestable<FungibleResourceIndicator> {
    override val samples: List<Sample<FungibleResourceIndicator>>
        get() = listOf(FungibleResourceIndicator.sample)

    @Test
    fun testAmount() {
        assertEquals(1.toDecimal192(), FungibleResourceIndicator.sample().amount)
    }
}