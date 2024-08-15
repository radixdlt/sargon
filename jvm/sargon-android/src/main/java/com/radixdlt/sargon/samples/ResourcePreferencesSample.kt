package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ResourcePreferences
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newResourcePreferencesSample
import com.radixdlt.sargon.newResourcePreferencesSampleOther

@UsesSampleValues
val ResourcePreferences.Companion.sample: Sample<ResourcePreferences>
    get() = object : Sample<ResourcePreferences> {

        override fun invoke(): ResourcePreferences = newResourcePreferencesSample()

        override fun other(): ResourcePreferences = newResourcePreferencesSampleOther()
    }