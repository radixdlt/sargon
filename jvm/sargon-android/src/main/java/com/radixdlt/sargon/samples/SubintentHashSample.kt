package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SubintentHash
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSubintentHashSample
import com.radixdlt.sargon.newSubintentHashSampleOther

@UsesSampleValues
val SubintentHash.Companion.sample: Sample<SubintentHash>
    get() = object : Sample<SubintentHash> {

        override fun invoke(): SubintentHash = newSubintentHashSample()

        override fun other(): SubintentHash = newSubintentHashSampleOther()
    }