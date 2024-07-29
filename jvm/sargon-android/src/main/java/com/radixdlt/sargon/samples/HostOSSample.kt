package com.radixdlt.sargon.samples

import com.radixdlt.sargon.HostOs
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHostOsSample
import com.radixdlt.sargon.newHostOsSampleOther

@UsesSampleValues
val HostOs.Companion.sample: Sample<HostOs>
    get() = object : Sample<HostOs> {

        override fun invoke(): HostOs = newHostOsSample()

        override fun other(): HostOs = newHostOsSampleOther()
    }