package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.from
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.vendor
import com.radixdlt.sargon.extensions.version
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DeviceInfoTest: SampleTestable<DeviceInfo> {
    override val samples: List<Sample<DeviceInfo>>
        get() = listOf(DeviceInfo.sample)

    @Test
    fun testNewDeviceInfoFromHostInfo() {
        val hostId = HostId.sample()
        val hostInfo = HostInfo.sample()

        val deviceInfo = DeviceInfo.from(hostId, hostInfo)

        assertEquals(
            hostId.id,
            deviceInfo.id
        )
        assertEquals(
            hostId.generatedAt,
            deviceInfo.date
        )
        assertEquals(
            hostInfo.hostAppVersion,
            deviceInfo.hostAppVersion
        )
        assertEquals(
            hostInfo.description.string,
            deviceInfo.description
        )
        assertEquals(
            hostInfo.hostOs.version,
            deviceInfo.systemVersion
        )
        assertEquals(
            hostInfo.hostOs.vendor,
            deviceInfo.hostVendor
        )
    }
}