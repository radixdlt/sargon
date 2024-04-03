package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.ResourceIndicator
import com.radixdlt.sargon.resourceIndicatorGetAddress

val ResourceIndicator.address: ResourceAddress
    get() = resourceIndicatorGetAddress(indicator = this)