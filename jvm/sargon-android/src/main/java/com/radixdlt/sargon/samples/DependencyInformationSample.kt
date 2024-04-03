package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.DependencyInformation
import com.radixdlt.sargon.newDependencyInformationSample
import com.radixdlt.sargon.newDependencyInformationSampleOther

@UsesSampleValues
val DependencyInformation.Companion.sample: Sample<DependencyInformation>
    get() = object : Sample<DependencyInformation> {
        override fun invoke(): DependencyInformation = newDependencyInformationSample()

        override fun other(): DependencyInformation = newDependencyInformationSampleOther()

    }