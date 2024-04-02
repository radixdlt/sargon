package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.newPackageAddressRandom
import com.radixdlt.sargon.newPoolAddressSampleMainnetMulti
import com.radixdlt.sargon.newPoolAddressSampleMainnetSingle
import com.radixdlt.sargon.newPoolAddressSampleMainnetTwo
import com.radixdlt.sargon.newPoolAddressRandom
import com.radixdlt.sargon.newPoolAddressSampleStokenetMulti
import com.radixdlt.sargon.newPoolAddressSampleStokenetSingle
import com.radixdlt.sargon.newPoolAddressSampleStokenetTwo

@UsesSampleValues
object PoolAddressSampleMainnet: SampleWithRandomValues<PoolAddress> {
    override fun invoke(): PoolAddress = single

    override fun other(): PoolAddress = two

    override fun random(): PoolAddress = newPoolAddressRandom(networkId = NetworkId.MAINNET)

    val single: PoolAddress
        get() = newPoolAddressSampleMainnetSingle()

    val two: PoolAddress
        get() = newPoolAddressSampleMainnetTwo()

    val multi: PoolAddress
        get() = newPoolAddressSampleMainnetMulti()
}

@UsesSampleValues
object PoolAddressSampleStokenet: SampleWithRandomValues<PoolAddress> {
    override fun invoke(): PoolAddress = single

    override fun other(): PoolAddress = two

    override fun random(): PoolAddress = newPoolAddressRandom(networkId = NetworkId.STOKENET)

    val single: PoolAddress
        get() = newPoolAddressSampleStokenetSingle()

    val two: PoolAddress
        get() = newPoolAddressSampleStokenetTwo()

    val multi: PoolAddress
        get() = newPoolAddressSampleStokenetMulti()
}

@UsesSampleValues
val PoolAddress.Companion.sampleMainnet: PoolAddressSampleMainnet
    get() = PoolAddressSampleMainnet

@UsesSampleValues
val PoolAddress.Companion.sampleStokenet: PoolAddressSampleStokenet
    get() = PoolAddressSampleStokenet

@UsesSampleValues
fun PoolAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newPoolAddressRandom(networkId = networkId)