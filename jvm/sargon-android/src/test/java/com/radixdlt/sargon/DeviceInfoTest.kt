package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DeviceInfoTest: SampleTestable<DeviceInfo> {
    override val samples: List<Sample<DeviceInfo>>
        get() = listOf(DeviceInfo.sample)

    @Test
    fun testJsonRoundtrip() {
        val device = DeviceInfo.sample()

        assertEquals(device, DeviceInfo.fromJson(device.toJson()))
    }
}