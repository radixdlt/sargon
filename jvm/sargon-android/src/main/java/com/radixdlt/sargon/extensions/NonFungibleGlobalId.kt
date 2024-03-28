package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.newNonFungibleGlobalIdFromString
import com.radixdlt.sargon.nonFungibleGlobalIdToString

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.init(globalId: String) =
    newNonFungibleGlobalIdFromString(string = globalId)

val NonFungibleGlobalId.string: String
    get() = nonFungibleGlobalIdToString(globalId = this)