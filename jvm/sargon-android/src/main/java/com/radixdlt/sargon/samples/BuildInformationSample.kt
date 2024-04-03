package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@UsesSampleValues
val SargonBuildInformation.Companion.sample: Sample<SargonBuildInformation>
    get() = object : Sample<SargonBuildInformation> {

        override fun invoke(): SargonBuildInformation = newSargonBuildInformationSample()

        override fun other(): SargonBuildInformation = newSargonBuildInformationSampleOther()
    }