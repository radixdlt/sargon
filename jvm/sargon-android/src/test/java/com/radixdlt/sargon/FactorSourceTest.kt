package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class FactorSourceTest : SampleTestable<FactorSource> {

    override val samples: List<Sample<FactorSource>>
        get() = listOf(FactorSource.sample)

    @Test
    fun testKind() {
        assertEquals(
            FactorSourceKind.DEVICE,
            FactorSource.sample().kind
        )

        assertEquals(
            FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET,
            FactorSource.sample.other().kind
        )
    }

}