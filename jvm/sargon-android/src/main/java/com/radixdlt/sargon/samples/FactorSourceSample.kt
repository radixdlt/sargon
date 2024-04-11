package com.radixdlt.sargon.samples

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSources
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newFactorSourceSample
import com.radixdlt.sargon.newFactorSourceSampleOther
import com.radixdlt.sargon.newFactorSourcesSample
import com.radixdlt.sargon.newFactorSourcesSampleOther

@UsesSampleValues
val FactorSource.Companion.sample: Sample<FactorSource>
    get() = object : Sample<FactorSource> {
        override fun invoke(): FactorSource = newFactorSourceSample()

        override fun other(): FactorSource = newFactorSourceSampleOther()
    }