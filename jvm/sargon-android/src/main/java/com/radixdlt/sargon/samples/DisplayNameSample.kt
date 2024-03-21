package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.newDisplayNameSample
import com.radixdlt.sargon.newDisplayNameSampleOther

@VisibleForTesting
val DisplayName.Companion.sample: Sample<DisplayName>
    get() = object : Sample<DisplayName> {

        override fun invoke(): DisplayName = newDisplayNameSample()

        override fun other(): DisplayName = newDisplayNameSampleOther()
    }

class DisplayNamePreviewParameterProvider : PreviewParameterProvider<DisplayName> {
    override val values: Sequence<DisplayName>
        get() = DisplayName.sample.all.asSequence()
}