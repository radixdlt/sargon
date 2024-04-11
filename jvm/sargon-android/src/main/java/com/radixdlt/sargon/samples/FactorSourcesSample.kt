package com.radixdlt.sargon.samples

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSources
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val FactorSources.Companion.sample: Sample<FactorSources>
    get() = object : Sample<FactorSources> {
        override fun invoke(): FactorSources = FactorSources.init(
            listOf(FactorSource.sample(), FactorSource.sample.other())
        )

        override fun other(): FactorSources = FactorSources.init(
            FactorSource.sample.other()
        )
    }