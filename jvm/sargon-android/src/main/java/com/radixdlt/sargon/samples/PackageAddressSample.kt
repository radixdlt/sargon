package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.newPackageAddressSampleMainnet
import com.radixdlt.sargon.newPackageAddressSampleMainnetOther
import com.radixdlt.sargon.newPackageAddressSampleRandom
import com.radixdlt.sargon.newPackageAddressSampleStokenet
import com.radixdlt.sargon.newPackageAddressSampleStokenetOther

@VisibleForTesting
object PackageAddressSampleMainnet: SampleWithRandomValues<PackageAddress> {
    override fun invoke(): PackageAddress = newPackageAddressSampleMainnet()

    override fun other(): PackageAddress = newPackageAddressSampleMainnetOther()

    override fun random(): PackageAddress = newPackageAddressSampleRandom(
        networkId = NetworkId.MAINNET
    )
}

@VisibleForTesting
val PackageAddress.Companion.sampleMainnet: PackageAddressSampleMainnet
    get() = PackageAddressSampleMainnet

@VisibleForTesting
object PackageAddressSampleStokenet: SampleWithRandomValues<PackageAddress> {
    override fun invoke(): PackageAddress = newPackageAddressSampleStokenet()

    override fun other(): PackageAddress = newPackageAddressSampleStokenetOther()

    override fun random(): PackageAddress = newPackageAddressSampleRandom(
        networkId = NetworkId.STOKENET
    )
}

@VisibleForTesting
val PackageAddress.Companion.sampleStokenet: PackageAddressSampleStokenet
    get() = PackageAddressSampleStokenet