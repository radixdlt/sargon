package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ids
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleResourceIndicatorTest: SampleTestable<NonFungibleResourceIndicator> {
    override val samples: List<Sample<NonFungibleResourceIndicator>>
        get() = listOf(NonFungibleResourceIndicator.sample)

    @Test
    fun testIds() {
        assertEquals(
            listOf(
                NonFungibleLocalId.init("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"),
            ),
            NonFungibleResourceIndicator.sample().ids
        )
    }
}