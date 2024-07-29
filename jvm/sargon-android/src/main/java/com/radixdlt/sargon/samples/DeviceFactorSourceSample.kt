package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.newDeviceFactorSourceSample
import com.radixdlt.sargon.newDeviceFactorSourceSampleOther

@UsesSampleValues
val DeviceFactorSource.Companion.sample: Sample<DeviceFactorSource>
    get() = object : Sample<DeviceFactorSource> {

        override fun invoke(): DeviceFactorSource = newDeviceFactorSourceSample()

        override fun other(): DeviceFactorSource = newDeviceFactorSourceSampleOther()
    }