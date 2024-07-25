package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DeviceInfoDescriptionTest: SampleTestable<DeviceInfoDescription> {
    override val samples: List<Sample<DeviceInfoDescription>>
        get() = listOf(DeviceInfoDescription.sample)

    @Test
    fun test_string() {
        val sut = DeviceInfoDescription(
            name = "My phone",
            model = "Brand name"
        )

        assertEquals(
            "My phone (Brand name)",
            sut.string
        )
    }
}