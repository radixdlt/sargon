package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.NonFungibleResourceAddress
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.newNonFungibleGlobalId
import com.radixdlt.sargon.newNonFungibleGlobalIdFromString

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.init(globalId: String) =
    newNonFungibleGlobalIdFromString(string = globalId)

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.init(nonFungibleResourceAddress: NonFungibleResourceAddress, nonFungibleLocalId: NonFungibleLocalId) =
    newNonFungibleGlobalId(nonFungibleResourceAddress, nonFungibleLocalId)

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.init(resourceAddress: ResourceAddress, nonFungibleLocalId: NonFungibleLocalId) =
    init(NonFungibleResourceAddress(resourceAddress), nonFungibleLocalId)

val NonFungibleGlobalId.string: String
    get() = this.asString

fun NonFungibleGlobalId.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)