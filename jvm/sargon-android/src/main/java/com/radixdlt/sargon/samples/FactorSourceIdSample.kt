package com.radixdlt.sargon.samples

import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newFactorSourceIdSample
import com.radixdlt.sargon.newFactorSourceIdSampleOther

@UsesSampleValues
val FactorSourceId.Companion.sample: Sample<FactorSourceId>
    get() = object : Sample<FactorSourceId> {
        override fun invoke(): FactorSourceId = newFactorSourceIdSample()

        override fun other(): FactorSourceId = newFactorSourceIdSampleOther()

    }