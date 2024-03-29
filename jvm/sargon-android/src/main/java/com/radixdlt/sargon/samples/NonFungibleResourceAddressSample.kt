package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NonFungibleResourceAddress
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleMainnet
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleMainnetOther
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleStokenet
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleStokenetOther

@VisibleForTesting
val NonFungibleResourceAddress.Companion.sampleMainnet: Sample<NonFungibleResourceAddress>
    get() = object : Sample<NonFungibleResourceAddress> {

        override fun invoke(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleMainnet()

        override fun other(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleMainnetOther()

    }

@VisibleForTesting
val NonFungibleResourceAddress.Companion.sampleStokenet: Sample<NonFungibleResourceAddress>
    get() = object : Sample<NonFungibleResourceAddress> {

        override fun invoke(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleStokenet()

        override fun other(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleStokenetOther()

    }