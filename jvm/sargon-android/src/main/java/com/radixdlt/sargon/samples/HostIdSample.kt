package com.radixdlt.sargon.samples

import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHostIdSample
import com.radixdlt.sargon.newHostIdSampleOther

@UsesSampleValues
val HostId.Companion.sample: Sample<HostId>
    get() = object : Sample<HostId> {

        override fun invoke(): HostId = newHostIdSample()

        override fun other(): HostId = newHostIdSampleOther()
    }