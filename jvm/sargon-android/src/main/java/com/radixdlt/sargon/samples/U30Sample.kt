package com.radixdlt.sargon.samples

import com.radixdlt.sargon.U30
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newU30Sample
import com.radixdlt.sargon.newU30SampleOther

@UsesSampleValues
val U30.Companion.sample: Sample<U30>
    get() = object : Sample<U30> {
        override fun invoke(): U30 = newU30Sample()

        override fun other(): U30 = newU30SampleOther()
    }

