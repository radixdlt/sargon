package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.NonFungibleResourceIndicator
import com.radixdlt.sargon.nonFungibleResourceIndicatorGetIds

val NonFungibleResourceIndicator.ids: List<NonFungibleLocalId>
    get() = nonFungibleResourceIndicatorGetIds(indicator = this)