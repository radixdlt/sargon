package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.PoolKind
import com.radixdlt.sargon.newPoolAddress
import com.radixdlt.sargon.poolAddressBech32Address
import com.radixdlt.sargon.poolAddressKind
import com.radixdlt.sargon.poolAddressNetworkId

@Throws(SargonException::class)
fun PoolAddress.Companion.init(validatingAddress: String) =
    newPoolAddress(bech32 = validatingAddress)

val PoolAddress.string: String
    get() = poolAddressBech32Address(address = this)

val PoolAddress.networkId: NetworkId
    get() = poolAddressNetworkId(address = this)

/**
 * Returns the kind of pool, either 1, 2 or Multi resources.
 */
val PoolAddress.poolKind: PoolKind
    get() = poolAddressKind(address = this)