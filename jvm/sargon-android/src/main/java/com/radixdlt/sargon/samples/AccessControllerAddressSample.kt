package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newAccessControllerAddressSampleMainnet
import com.radixdlt.sargon.newAccessControllerAddressSampleMainnetOther
import com.radixdlt.sargon.newAccessControllerAddressRandom
import com.radixdlt.sargon.newAccessControllerAddressSampleStokenet
import com.radixdlt.sargon.newAccessControllerAddressSampleStokenetOther

@UsesSampleValues
object AccessControllerAddressSampleMainnet: SampleWithRandomValues<AccessControllerAddress> {

    override fun invoke(): AccessControllerAddress =
        newAccessControllerAddressSampleMainnet()

    override fun other(): AccessControllerAddress =
        newAccessControllerAddressSampleMainnetOther()

    override fun random(): AccessControllerAddress = newAccessControllerAddressRandom(
        networkId = NetworkId.MAINNET
    )

}

@UsesSampleValues
val AccessControllerAddress.Companion.sampleMainnet: AccessControllerAddressSampleMainnet
    get() = AccessControllerAddressSampleMainnet

@UsesSampleValues
object AccessControllerAddressSampleStokenet: SampleWithRandomValues<AccessControllerAddress> {

    override fun invoke(): AccessControllerAddress =
        newAccessControllerAddressSampleStokenet()

    override fun other(): AccessControllerAddress =
        newAccessControllerAddressSampleStokenetOther()

    override fun random(): AccessControllerAddress = newAccessControllerAddressRandom(
        networkId = NetworkId.STOKENET
    )
}

@UsesSampleValues
val AccessControllerAddress.Companion.sampleStokenet: AccessControllerAddressSampleStokenet
    get() = AccessControllerAddressSampleStokenet

@UsesSampleValues
fun AccessControllerAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newAccessControllerAddressRandom(networkId = networkId)