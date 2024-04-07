package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Decimal192
import com.radixdlt.sargon.FungibleResourceIndicator
import com.radixdlt.sargon.fungibleResourceIndicatorGetAmount

val FungibleResourceIndicator.amount: Decimal192
    get() = fungibleResourceIndicatorGetAmount(indicator = this)