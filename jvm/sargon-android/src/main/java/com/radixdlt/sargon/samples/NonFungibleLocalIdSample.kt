package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdRandom
import com.radixdlt.sargon.newNonFungibleLocalIdSample
import com.radixdlt.sargon.newNonFungibleLocalIdSampleOther

@UsesSampleValues
val NonFungibleLocalId.Companion.sample: SampleWithRandomValues<NonFungibleLocalId>
    get() = object : SampleWithRandomValues<NonFungibleLocalId> {

        override fun invoke(): NonFungibleLocalId = newNonFungibleLocalIdSample()

        override fun other(): NonFungibleLocalId = newNonFungibleLocalIdSampleOther()

        override fun random(): NonFungibleLocalId = newNonFungibleLocalIdRandom()
    }