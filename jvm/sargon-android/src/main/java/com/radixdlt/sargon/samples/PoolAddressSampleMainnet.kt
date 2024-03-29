package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.newPackageAddressSampleRandom
import com.radixdlt.sargon.newPoolAddressSampleMainnetMulti
import com.radixdlt.sargon.newPoolAddressSampleMainnetSingle
import com.radixdlt.sargon.newPoolAddressSampleMainnetTwo
import com.radixdlt.sargon.newPoolAddressSampleRandom
import com.radixdlt.sargon.newPoolAddressSampleStokenetMulti
import com.radixdlt.sargon.newPoolAddressSampleStokenetSingle
import com.radixdlt.sargon.newPoolAddressSampleStokenetTwo

@VisibleForTesting
object PoolAddressSampleMainnet: SampleWithRandomValues<PoolAddress> {
    override fun invoke(): PoolAddress = single

    override fun other(): PoolAddress = two

    override fun random(): PoolAddress = newPoolAddressSampleRandom(networkId = NetworkId.MAINNET)

    val single: PoolAddress
        get() = newPoolAddressSampleMainnetSingle()

    val two: PoolAddress
        get() = newPoolAddressSampleMainnetTwo()

    val multi: PoolAddress
        get() = newPoolAddressSampleMainnetMulti()
}

@VisibleForTesting
object PoolAddressSampleStokenet: SampleWithRandomValues<PoolAddress> {
    override fun invoke(): PoolAddress = single

    override fun other(): PoolAddress = two

    override fun random(): PoolAddress = newPoolAddressSampleRandom(networkId = NetworkId.STOKENET)

    val single: PoolAddress
        get() = newPoolAddressSampleStokenetSingle()

    val two: PoolAddress
        get() = newPoolAddressSampleStokenetTwo()

    val multi: PoolAddress
        get() = newPoolAddressSampleStokenetMulti()
}

@VisibleForTesting
val PoolAddress.Companion.sampleMainnet: PoolAddressSampleMainnet
    get() = PoolAddressSampleMainnet

@VisibleForTesting
val PoolAddress.Companion.sampleStokenet: PoolAddressSampleStokenet
    get() = PoolAddressSampleStokenet

@VisibleForTesting
fun PoolAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newPoolAddressSampleRandom(networkId = networkId)