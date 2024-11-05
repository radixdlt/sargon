package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.ComponentAddressKind
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newComponentAddress

@Throws(SargonException::class)
fun ComponentAddress.Companion.init(validatingAddress: String) =
    newComponentAddress(bech32 = validatingAddress)

val ComponentAddress.string: String
    get() = this.address

/**
 * If the `EntityType == GlobalGenericComponent`
 */
val ComponentAddress.isGlobal: Boolean
    get() = this.kind == ComponentAddressKind.GLOBAL

/**
 * If the `EntityType == InternalGenericComponent`
 */
val ComponentAddress.isInternal: Boolean
    get() = this.kind == ComponentAddressKind.GLOBAL

fun ComponentAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun ComponentAddress.asGeneral(): Address.Component = Address.Component(this)