package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.HdPathComponent
import com.radixdlt.sargon.hdPathComponentGetNonHardenedValue

val HdPathComponent.nonHardenedValue: HDPathValue
    get() = hdPathComponentGetNonHardenedValue(component = this)