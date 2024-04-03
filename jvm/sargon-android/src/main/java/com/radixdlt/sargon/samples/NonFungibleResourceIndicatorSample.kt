package com.radixdlt.sargon.samples

import com.radixdlt.sargon.NonFungibleResourceIndicator
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newNonFungibleResourceIndicatorSample
import com.radixdlt.sargon.newNonFungibleResourceIndicatorSampleOther

@UsesSampleValues
val NonFungibleResourceIndicator.Companion.sample: Sample<NonFungibleResourceIndicator>
    get() = object : Sample<NonFungibleResourceIndicator> {
        override fun invoke(): NonFungibleResourceIndicator =
            newNonFungibleResourceIndicatorSample()

        override fun other(): NonFungibleResourceIndicator =
            newNonFungibleResourceIndicatorSampleOther()
    }