package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.nonFungibleLocalIdAsStr


val NonFungibleLocalId.string: String
    get() = nonFungibleLocalIdAsStr(id = this)