package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdSample
import com.radixdlt.sargon.newNonFungibleLocalIdSampleOther


@VisibleForTesting
val NonFungibleLocalId.Companion.sample: Sample<NonFungibleLocalId>
    get() = object : Sample<NonFungibleLocalId> {

        override fun invoke(): NonFungibleLocalId = newNonFungibleLocalIdSample()

        override fun other(): NonFungibleLocalId = newNonFungibleLocalIdSampleOther()
    }

class NonFungibleLocalIdPreviewParameterProvider : PreviewParameterProvider<NonFungibleLocalId> {
    override val values: Sequence<NonFungibleLocalId>
        get() = NonFungibleLocalId.sample.all.asSequence()
}