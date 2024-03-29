package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.newNonFungibleGlobalIdSample
import com.radixdlt.sargon.newNonFungibleGlobalIdSampleOther


@VisibleForTesting
val NonFungibleGlobalId.Companion.sample: Sample<NonFungibleGlobalId>
    get() = object : Sample<NonFungibleGlobalId> {

        override fun invoke(): NonFungibleGlobalId = newNonFungibleGlobalIdSample()

        override fun other(): NonFungibleGlobalId = newNonFungibleGlobalIdSampleOther()
    }

class NonFungibleGlobalIdPreviewParameterProvider : PreviewParameterProvider<NonFungibleGlobalId> {
    override val values: Sequence<NonFungibleGlobalId>
        get() = NonFungibleGlobalId.sample.all.asSequence()
}