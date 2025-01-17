package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.days
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class TimePeriodTest: SampleTestable<TimePeriod> {

    override val samples: List<Sample<TimePeriod>>
        get() = listOf(TimePeriod.sample)

    @Test
    fun test() {
        assertEquals(
            TimePeriod.init(1),
            TimePeriod.sample()
        )
        assertEquals(
            TimePeriod.init(5).days,
            5
        )
    }
}