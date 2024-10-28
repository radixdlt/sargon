package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.PoolKind
import com.radixdlt.sargon.newPoolAddress

@Throws(SargonException::class)
fun PoolAddress.Companion.init(validatingAddress: String) =
    newPoolAddress(bech32 = validatingAddress)

val PoolAddress.string: String
    get() = this.address

/**
 * Returns the kind of pool, either 1, 2 or Multi resources.
 */
val PoolAddress.poolKind: PoolKind
    get() = this.kind

fun PoolAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun PoolAddress.asGeneral(): Address.Pool = Address.Pool(this)
