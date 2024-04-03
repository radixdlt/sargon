package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AppPreferences
import com.radixdlt.sargon.newAppPreferencesSample
import com.radixdlt.sargon.newAppPreferencesSampleOther

@UsesSampleValues
val AppPreferences.Companion.sample: Sample<AppPreferences>
    get() = object : Sample<AppPreferences> {

        override fun invoke(): AppPreferences = newAppPreferencesSample()

        override fun other(): AppPreferences = newAppPreferencesSampleOther()
    }