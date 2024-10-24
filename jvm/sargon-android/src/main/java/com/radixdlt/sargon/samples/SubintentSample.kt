package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Subintent
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSubintentSample
import com.radixdlt.sargon.newSubintentSampleOther

@UsesSampleValues
val Subintent.Companion.sample: Sample<Subintent>
    get() = object : Sample<Subintent> {

        override fun invoke(): Subintent = newSubintentSample()

        override fun other(): Subintent = newSubintentSampleOther()
    }