package com.radixdlt.sargon.samples

import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHostInfoSample
import com.radixdlt.sargon.newHostInfoSampleOther

@UsesSampleValues
val HostInfo.Companion.sample: Sample<HostInfo>
    get() = object : Sample<HostInfo> {

        override fun invoke(): HostInfo = newHostInfoSample()

        override fun other(): HostInfo = newHostInfoSampleOther()
    }