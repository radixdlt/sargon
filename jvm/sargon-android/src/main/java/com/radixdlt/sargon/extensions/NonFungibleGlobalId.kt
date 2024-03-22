package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.newNonFungibleGlobalIdFromString

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.from(globalId: String) =
    newNonFungibleGlobalIdFromString(globalId = globalId)