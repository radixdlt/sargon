package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.newNonFungibleGlobalIdSample
import com.radixdlt.sargon.newNonFungibleGlobalIdSampleOther

@UsesSampleValues
val NonFungibleGlobalId.Companion.sample: Sample<NonFungibleGlobalId>
    get() = object : Sample<NonFungibleGlobalId> {

        override fun invoke(): NonFungibleGlobalId = newNonFungibleGlobalIdSample()

        override fun other(): NonFungibleGlobalId = newNonFungibleGlobalIdSampleOther()
    }