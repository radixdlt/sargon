package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdBytes
import com.radixdlt.sargon.newNonFungibleLocalIdInt
import com.radixdlt.sargon.newNonFungibleLocalIdRuid
import com.radixdlt.sargon.newNonFungibleLocalIdString
import com.radixdlt.sargon.nonFungibleLocalIdAsStr

fun NonFungibleLocalId.Companion.fromBytes(bytes: BagOfBytes): NonFungibleLocalId =
    newNonFungibleLocalIdBytes(bytes = bytes)

fun NonFungibleLocalId.Companion.fromInt(value: ULong): NonFungibleLocalId =
    newNonFungibleLocalIdInt(value = value)

fun NonFungibleLocalId.Companion.fromRuid(value: BagOfBytes): NonFungibleLocalId =
    newNonFungibleLocalIdRuid(bytes = value)

fun NonFungibleLocalId.Companion.fromString(string: String): NonFungibleLocalId =
    newNonFungibleLocalIdString(string = string)

val NonFungibleLocalId.string: String
    get() = nonFungibleLocalIdAsStr(id = this)