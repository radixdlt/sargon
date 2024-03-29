package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdBytes
import com.radixdlt.sargon.newNonFungibleLocalIdFromString
import com.radixdlt.sargon.newNonFungibleLocalIdInt
import com.radixdlt.sargon.newNonFungibleLocalIdRuid
import com.radixdlt.sargon.newNonFungibleLocalIdString
import com.radixdlt.sargon.nonFungibleGlobalIdFormatted
import com.radixdlt.sargon.nonFungibleLocalIdAsStr
import com.radixdlt.sargon.nonFungibleLocalIdFormatted

@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.init(localId: String): NonFungibleLocalId =
    newNonFungibleLocalIdFromString(localId = localId)

@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.fromBytes(bytes: BagOfBytes): NonFungibleLocalId =
    newNonFungibleLocalIdBytes(bytes = bytes)

fun NonFungibleLocalId.Companion.fromInt(value: ULong): NonFungibleLocalId =
    newNonFungibleLocalIdInt(value = value)

@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.fromRuid(value: BagOfBytes): NonFungibleLocalId =
    newNonFungibleLocalIdRuid(bytes = value)

@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.fromString(string: String): NonFungibleLocalId =
    newNonFungibleLocalIdString(string = string)

val NonFungibleLocalId.string: String
    get() = nonFungibleLocalIdAsStr(id = this)

fun NonFungibleLocalId.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = nonFungibleLocalIdFormatted(id = this, format = format)