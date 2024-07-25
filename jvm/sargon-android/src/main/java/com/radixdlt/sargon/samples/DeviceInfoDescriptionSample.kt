package com.radixdlt.sargon.samples

import com.radixdlt.sargon.DeviceInfoDescription
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newDeviceInfoDescriptionSample
import com.radixdlt.sargon.newDeviceInfoDescriptionSampleOther

@UsesSampleValues
val DeviceInfoDescription.Companion.sample: Sample<DeviceInfoDescription>
    get() = object : Sample<DeviceInfoDescription> {
        override fun invoke(): DeviceInfoDescription = newDeviceInfoDescriptionSample()

        override fun other(): DeviceInfoDescription = newDeviceInfoDescriptionSampleOther()
    }