package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DependencyInformationTest: SampleTestable<DependencyInformation> {
    override val samples: List<Sample<DependencyInformation>>
        get() = listOf(DependencyInformation.sample)

    @Test
    fun testString() {
        assertEquals("main", DependencyInformation.sample().string)
    }
}