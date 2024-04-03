package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Header
import com.radixdlt.sargon.newHeaderSample
import com.radixdlt.sargon.newHeaderSampleOther

@UsesSampleValues
val Header.Companion.sample: Sample<Header>
    get() = object : Sample<Header> {

        override fun invoke(): Header = newHeaderSample()

        override fun other(): Header = newHeaderSampleOther()

    }