package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.newNonFungibleGlobalIdFromString

@Throws(SargonException::class)
fun NonFungibleGlobalId.Companion.init(globalId: String) =
    newNonFungibleGlobalIdFromString(globalId = globalId)