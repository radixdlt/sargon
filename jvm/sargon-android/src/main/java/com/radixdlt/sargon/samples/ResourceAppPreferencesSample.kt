package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.ResourceAppPreferences
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.newResourcePreferencesSample
import com.radixdlt.sargon.newResourcePreferencesSampleOther

@UsesSampleValues
val ResourceAppPreferences.Companion.sample: Sample<ResourceAppPreferences>
    get() = object : Sample<ResourceAppPreferences> {

        override fun invoke(): ResourceAppPreferences = newResourcePreferencesSample().asIdentifiable()

        override fun other(): ResourceAppPreferences = newResourcePreferencesSampleOther().asIdentifiable()
    }