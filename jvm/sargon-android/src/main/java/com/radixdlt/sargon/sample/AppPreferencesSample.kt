package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AppPreferences
import com.radixdlt.sargon.newAppPreferencesSample
import com.radixdlt.sargon.newAppPreferencesSampleOther

@VisibleForTesting
val AppPreferences.Companion.sample: Sample<AppPreferences>
    get() = object : Sample<AppPreferences> {

        override fun invoke(): AppPreferences = newAppPreferencesSample()

        override fun other(): AppPreferences = newAppPreferencesSampleOther()
    }

class AppPreferencesPreviewParameterProvider: PreviewParameterProvider<AppPreferences> {
    override val values: Sequence<AppPreferences>
        get() = AppPreferences.sample.all.asSequence()

}