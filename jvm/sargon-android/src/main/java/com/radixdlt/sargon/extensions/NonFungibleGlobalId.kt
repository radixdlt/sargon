package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.LegacyOlympiaAccountAddress
import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.legacyOlympiaAccountAddressFormatted
import com.radixdlt.sargon.newNonFungibleGlobalIdFromString
import com.radixdlt.sargon.nonFungibleGlobalIdFormatted
import com.radixdlt.sargon.nonFungibleGlobalIdToString

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.init(globalId: String) =
    newNonFungibleGlobalIdFromString(string = globalId)

val NonFungibleGlobalId.string: String
    get() = nonFungibleGlobalIdToString(globalId = this)

fun NonFungibleGlobalId.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = nonFungibleGlobalIdFormatted(globalId = this, format = format)