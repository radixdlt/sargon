package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.newPublicKeyHashSample
import com.radixdlt.sargon.newPublicKeyHashSampleOther

@VisibleForTesting
val PublicKeyHash.Companion.sample: Sample<PublicKeyHash>
    get() = object : Sample<PublicKeyHash> {

        override fun invoke(): PublicKeyHash = newPublicKeyHashSample()

        override fun other(): PublicKeyHash = newPublicKeyHashSampleOther()
    }

class PublicKeyHashPreviewParameterProvider: PreviewParameterProvider<PublicKeyHash> {
    override val values: Sequence<PublicKeyHash>
        get() = PublicKeyHash.sample.all.asSequence()

}