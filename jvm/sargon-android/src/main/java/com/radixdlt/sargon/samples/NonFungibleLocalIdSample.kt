package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdInt
import com.radixdlt.sargon.newNonFungibleLocalIdString


@VisibleForTesting
val NonFungibleLocalId.Companion.sample: Sample<NonFungibleLocalId>
    get() = object : Sample<NonFungibleLocalId> {

        override fun invoke(): NonFungibleLocalId = newNonFungibleLocalIdInt(value = 1337.toULong())

        override fun other(): NonFungibleLocalId = newNonFungibleLocalIdString("FOO")
    }

class NonFungibleLocalIdPreviewParameterProvider : PreviewParameterProvider<NonFungibleLocalId> {
    override val values: Sequence<NonFungibleLocalId>
        get() = NonFungibleLocalId.sample.all.asSequence()
}