package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.newBlobsSample
import com.radixdlt.sargon.newBlobsSampleOther

@VisibleForTesting
val Blobs.Companion.sample: Sample<Blobs>
    get() = object : Sample<Blobs> {

        override fun invoke(): Blobs = newBlobsSample()

        override fun other(): Blobs = newBlobsSampleOther()
    }