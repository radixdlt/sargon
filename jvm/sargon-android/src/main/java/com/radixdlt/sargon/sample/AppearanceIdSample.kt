package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AppearanceId
import com.radixdlt.sargon.newAppearanceIdSample
import com.radixdlt.sargon.newAppearanceIdSampleOther

@VisibleForTesting
val AppearanceId.Companion.sample: Sample<AppearanceId>
    get() = object : Sample<AppearanceId> {

        override fun invoke(): AppearanceId = newAppearanceIdSample()

        override fun other(): AppearanceId = newAppearanceIdSampleOther()
    }

class AppearanceIdPreviewParameterProvider: PreviewParameterProvider<AppearanceId> {
    override val values: Sequence<AppearanceId>
        get() = AppearanceId.sample.all.asSequence()

}