package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newAccountAddressRandom
import com.radixdlt.sargon.newIdentityAddressSampleMainnet
import com.radixdlt.sargon.newIdentityAddressSampleMainnetOther
import com.radixdlt.sargon.newIdentityAddressRandom
import com.radixdlt.sargon.newIdentityAddressSampleStokenet
import com.radixdlt.sargon.newIdentityAddressSampleStokenetOther

@UsesSampleValues
object IdentityAddressSampleMainnet: SampleWithRandomValues<IdentityAddress> {
    override fun invoke(): IdentityAddress = newIdentityAddressSampleMainnet()

    override fun other(): IdentityAddress = newIdentityAddressSampleMainnetOther()

    override fun random(): IdentityAddress = newIdentityAddressRandom(
        networkId = NetworkId.MAINNET
    )
}

@UsesSampleValues
val IdentityAddress.Companion.sampleMainnet: IdentityAddressSampleMainnet
    get() = IdentityAddressSampleMainnet

@UsesSampleValues
object IdentityAddressSampleStokenet: SampleWithRandomValues<IdentityAddress> {
    override fun invoke(): IdentityAddress = newIdentityAddressSampleStokenet()

    override fun other(): IdentityAddress = newIdentityAddressSampleStokenetOther()

    override fun random(): IdentityAddress = newIdentityAddressRandom(
        networkId = NetworkId.STOKENET
    )
}

@UsesSampleValues
val IdentityAddress.Companion.sampleStokenet: IdentityAddressSampleStokenet
    get() = IdentityAddressSampleStokenet

@UsesSampleValues
fun IdentityAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newIdentityAddressRandom(networkId = networkId)