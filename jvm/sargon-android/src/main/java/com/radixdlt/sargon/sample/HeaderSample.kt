package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Header
import com.radixdlt.sargon.newHeaderSample
import com.radixdlt.sargon.newHeaderSampleOther

@VisibleForTesting
val Header.Companion.sample: Sample<Header>
    get() = object : Sample<Header> {

        override fun invoke(): Header = newHeaderSample()

        override fun other(): Header = newHeaderSampleOther()

    }

class HeaderPreviewParameterProvider :
    PreviewParameterProvider<Header> {
    override val values: Sequence<Header>
        get() = Header.sample.all.asSequence()
}