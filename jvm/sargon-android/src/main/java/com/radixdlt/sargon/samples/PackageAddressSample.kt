package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.newAccountAddressRandom
import com.radixdlt.sargon.newPackageAddressSampleMainnet
import com.radixdlt.sargon.newPackageAddressSampleMainnetOther
import com.radixdlt.sargon.newPackageAddressRandom
import com.radixdlt.sargon.newPackageAddressSampleStokenet
import com.radixdlt.sargon.newPackageAddressSampleStokenetOther

@UsesSampleValues
object PackageAddressSampleMainnet: SampleWithRandomValues<PackageAddress> {
    override fun invoke(): PackageAddress = newPackageAddressSampleMainnet()

    override fun other(): PackageAddress = newPackageAddressSampleMainnetOther()

    override fun random(): PackageAddress = newPackageAddressRandom(
        networkId = NetworkId.MAINNET
    )
}

@UsesSampleValues
val PackageAddress.Companion.sampleMainnet: PackageAddressSampleMainnet
    get() = PackageAddressSampleMainnet

@UsesSampleValues
object PackageAddressSampleStokenet: SampleWithRandomValues<PackageAddress> {
    override fun invoke(): PackageAddress = newPackageAddressSampleStokenet()

    override fun other(): PackageAddress = newPackageAddressSampleStokenetOther()

    override fun random(): PackageAddress = newPackageAddressRandom(
        networkId = NetworkId.STOKENET
    )
}

@UsesSampleValues
val PackageAddress.Companion.sampleStokenet: PackageAddressSampleStokenet
    get() = PackageAddressSampleStokenet

@UsesSampleValues
fun PackageAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newPackageAddressRandom(networkId = networkId)