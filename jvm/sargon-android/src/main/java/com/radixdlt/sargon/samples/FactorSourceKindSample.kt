package com.radixdlt.sargon.samples

import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newFactorSourceKindSample
import com.radixdlt.sargon.newFactorSourceKindSampleOther

@UsesSampleValues
val FactorSourceKind.Companion.sample: Sample<FactorSourceKind>
    get() = object : Sample<FactorSourceKind> {
        override fun invoke(): FactorSourceKind = newFactorSourceKindSample()

        override fun other(): FactorSourceKind = newFactorSourceKindSampleOther()

    }