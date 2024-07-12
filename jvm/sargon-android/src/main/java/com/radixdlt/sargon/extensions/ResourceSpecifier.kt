
package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.ResourceSpecifier
import com.radixdlt.sargon.resourceSpecifierGetAddress

val ResourceSpecifier.address: ResourceAddress
    get() = resourceSpecifierGetAddress(specifier = this)
