package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newIdentityAddressSampleMainnet
import com.radixdlt.sargon.newIdentityAddressSampleMainnetOther
import com.radixdlt.sargon.newIdentityAddressSampleRandom
import com.radixdlt.sargon.newIdentityAddressSampleStokenet
import com.radixdlt.sargon.newIdentityAddressSampleStokenetOther

@VisibleForTesting
object IdentityAddressSampleMainnet: SampleWithRandomValues<IdentityAddress> {
    override fun invoke(): IdentityAddress = newIdentityAddressSampleMainnet()

    override fun other(): IdentityAddress = newIdentityAddressSampleMainnetOther()

    override fun random(): IdentityAddress = newIdentityAddressSampleRandom(
        networkId = NetworkId.MAINNET
    )
}

@VisibleForTesting
val IdentityAddress.Companion.sampleMainnet: IdentityAddressSampleMainnet
    get() = IdentityAddressSampleMainnet

@VisibleForTesting
object IdentityAddressSampleStokenet: SampleWithRandomValues<IdentityAddress> {
    override fun invoke(): IdentityAddress = newIdentityAddressSampleStokenet()

    override fun other(): IdentityAddress = newIdentityAddressSampleStokenetOther()

    override fun random(): IdentityAddress = newIdentityAddressSampleRandom(
        networkId = NetworkId.STOKENET
    )
}

@VisibleForTesting
val IdentityAddress.Companion.sampleStokenet: IdentityAddressSampleStokenet
    get() = IdentityAddressSampleStokenet