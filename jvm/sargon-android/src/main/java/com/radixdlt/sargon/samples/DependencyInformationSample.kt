package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.DependencyInformation
import com.radixdlt.sargon.newDependencyInformationSample
import com.radixdlt.sargon.newDependencyInformationSampleOther

@VisibleForTesting
val DependencyInformation.Companion.sample: Sample<DependencyInformation>
    get() = object : Sample<DependencyInformation> {
        override fun invoke(): DependencyInformation = newDependencyInformationSample()

        override fun other(): DependencyInformation = newDependencyInformationSampleOther()

    }