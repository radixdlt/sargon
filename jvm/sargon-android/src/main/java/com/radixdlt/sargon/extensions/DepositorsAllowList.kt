package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl
import com.radixdlt.sargon.ResourceOrNonFungible

class DepositorsAllowList private constructor(
    array: IdentifiedArray<ResourceOrNonFungible, ResourceOrNonFungible>
) : IdentifiedArray<ResourceOrNonFungible, ResourceOrNonFungible> by array {

    constructor(resourcesOrNonFungibles: List<ResourceOrNonFungible>) : this(
        IdentifiedArrayImpl(
            elements = resourcesOrNonFungibles,
            identifier = { it }
        )
    )

    constructor(vararg resourceOrNonFungible: ResourceOrNonFungible) : this(
        IdentifiedArrayImpl(element = resourceOrNonFungible, identifier = { it })
    )
}