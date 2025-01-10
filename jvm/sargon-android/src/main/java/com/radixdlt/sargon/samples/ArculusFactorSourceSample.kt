package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ArculusCardFactorSource
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newArculusCardFactorSourceSample
import com.radixdlt.sargon.newArculusCardFactorSourceSampleOther

@UsesSampleValues
val ArculusCardFactorSource.Companion.sample: Sample<ArculusCardFactorSource>
    get() = object : Sample<ArculusCardFactorSource> {
        override fun invoke(): ArculusCardFactorSource = newArculusCardFactorSourceSample()

        override fun other(): ArculusCardFactorSource = newArculusCardFactorSourceSampleOther()
    }