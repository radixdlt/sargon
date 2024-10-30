package com.radixdlt.sargon.samples

import com.radixdlt.sargon.U31
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newU31Sample
import com.radixdlt.sargon.newU31SampleOther

@UsesSampleValues
val U31.Companion.sample: Sample<U31>
    get() = object : Sample<U31> {
        override fun invoke(): U31 = newU31Sample()

        override fun other(): U31 = newU31SampleOther()
    }

