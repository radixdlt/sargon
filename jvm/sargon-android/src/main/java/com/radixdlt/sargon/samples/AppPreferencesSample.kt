package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.AppPreferences
import com.radixdlt.sargon.newAppPreferencesSample
import com.radixdlt.sargon.newAppPreferencesSampleOther

@VisibleForTesting
val AppPreferences.Companion.sample: Sample<AppPreferences>
    get() = object : Sample<AppPreferences> {

        override fun invoke(): AppPreferences = newAppPreferencesSample()

        override fun other(): AppPreferences = newAppPreferencesSampleOther()
    }