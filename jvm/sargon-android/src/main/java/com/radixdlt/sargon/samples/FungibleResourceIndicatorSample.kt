package com.radixdlt.sargon.samples

import com.radixdlt.sargon.FungibleResourceIndicator
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newFungibleResourceIndicatorSample
import com.radixdlt.sargon.newFungibleResourceIndicatorSampleOther

@UsesSampleValues
val FungibleResourceIndicator.Companion.sample: Sample<FungibleResourceIndicator>
    get() = object: Sample<FungibleResourceIndicator> {
        override fun invoke(): FungibleResourceIndicator = newFungibleResourceIndicatorSample()

        override fun other(): FungibleResourceIndicator = newFungibleResourceIndicatorSampleOther()
    }