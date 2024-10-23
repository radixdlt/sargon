package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SubintentManifest
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSubintentManifestSample
import com.radixdlt.sargon.newSubintentManifestSampleOther

@UsesSampleValues
val SubintentManifest.Companion.sample: Sample<SubintentManifest>
    get() = object : Sample<SubintentManifest> {

        override fun invoke(): SubintentManifest = newSubintentManifestSample()

        override fun other(): SubintentManifest = newSubintentManifestSampleOther()

    }