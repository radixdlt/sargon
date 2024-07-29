package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.DeviceInfo
import com.radixdlt.sargon.newDeviceInfoSample
import com.radixdlt.sargon.newDeviceInfoSampleOther

@UsesSampleValues
val DeviceInfo.Companion.sample: Sample<DeviceInfo>
    get() = object : Sample<DeviceInfo> {

        override fun invoke(): DeviceInfo = newDeviceInfoSample()

        override fun other(): DeviceInfo = newDeviceInfoSampleOther()
    }