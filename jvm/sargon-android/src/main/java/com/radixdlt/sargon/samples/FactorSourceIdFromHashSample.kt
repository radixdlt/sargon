package com.radixdlt.sargon.samples

import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newFactorSourceIdFromHashSample
import com.radixdlt.sargon.newFactorSourceIdFromHashSampleOther

@UsesSampleValues
val FactorSourceIdFromHash.Companion.sample: Sample<FactorSourceIdFromHash>
    get() = object : Sample<FactorSourceIdFromHash> {
        override fun invoke(): FactorSourceIdFromHash = newFactorSourceIdFromHashSample()

        override fun other(): FactorSourceIdFromHash = newFactorSourceIdFromHashSampleOther()

    }