package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.Header
import com.radixdlt.sargon.newHeaderSample
import com.radixdlt.sargon.newHeaderSampleOther

@VisibleForTesting
val Header.Companion.sample: Sample<Header>
    get() = object : Sample<Header> {

        override fun invoke(): Header = newHeaderSample()

        override fun other(): Header = newHeaderSampleOther()

    }