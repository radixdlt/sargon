package com.radixdlt.sargon.samples

import com.radixdlt.sargon.LockerAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newLockerAddressRandom
import com.radixdlt.sargon.newLockerAddressSampleMainnet
import com.radixdlt.sargon.newLockerAddressSampleMainnetOther
import com.radixdlt.sargon.newLockerAddressSampleStokenet
import com.radixdlt.sargon.newLockerAddressSampleStokenetOther

@UsesSampleValues
object LockerAddressSampleMainnet : SampleWithRandomValues<LockerAddress> {

    override fun invoke(): LockerAddress = newLockerAddressSampleMainnet()

    override fun other(): LockerAddress = newLockerAddressSampleMainnetOther()

    override fun random(): LockerAddress = newLockerAddressRandom(networkId = NetworkId.MAINNET)
}

@UsesSampleValues
val LockerAddress.Companion.sampleMainnet: LockerAddressSampleMainnet
    get() = LockerAddressSampleMainnet

@UsesSampleValues
object LockerAddressSampleStokenet : SampleWithRandomValues<LockerAddress> {

    override fun invoke(): LockerAddress = newLockerAddressSampleStokenet()

    override fun other(): LockerAddress = newLockerAddressSampleStokenetOther()

    override fun random(): LockerAddress = newLockerAddressRandom(
        networkId = NetworkId.STOKENET
    )
}

@UsesSampleValues
val LockerAddress.Companion.sampleStokenet: LockerAddressSampleStokenet
    get() = LockerAddressSampleStokenet

@UsesSampleValues
fun LockerAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newLockerAddressRandom(networkId = networkId)