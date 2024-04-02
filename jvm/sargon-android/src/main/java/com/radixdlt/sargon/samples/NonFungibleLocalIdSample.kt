package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdSample
import com.radixdlt.sargon.newNonFungibleLocalIdSampleOther

@UsesSampleValues
val NonFungibleLocalId.Companion.sample: Sample<NonFungibleLocalId>
    get() = object : Sample<NonFungibleLocalId> {

        override fun invoke(): NonFungibleLocalId = newNonFungibleLocalIdSample()

        override fun other(): NonFungibleLocalId = newNonFungibleLocalIdSampleOther()
    }