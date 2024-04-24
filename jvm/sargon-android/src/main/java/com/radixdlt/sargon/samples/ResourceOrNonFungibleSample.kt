package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ResourceOrNonFungible
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newResourceOrNonFungibleSample
import com.radixdlt.sargon.newResourceOrNonFungibleSampleOther

@UsesSampleValues
val ResourceOrNonFungible.Companion.sample: Sample<ResourceOrNonFungible>
    get() = object : Sample<ResourceOrNonFungible> {
        override fun invoke(): ResourceOrNonFungible = newResourceOrNonFungibleSample()

        override fun other(): ResourceOrNonFungible = newResourceOrNonFungibleSampleOther()

    }