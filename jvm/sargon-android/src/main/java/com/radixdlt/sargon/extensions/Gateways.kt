package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl
import com.radixdlt.sargon.Url

class Gateways private constructor(
    array: IdentifiedArray<Url, Gateway>
) : IdentifiedArray<Url, Gateway> by array {

    constructor(gateways: List<Gateway>) : this(
        IdentifiedArrayImpl(
            elements = gateways,
            identifier = { it.url }
        )
    )

    constructor(vararg account: Gateway) : this(
        IdentifiedArrayImpl(element = account, identifier = { it.url })
    )
}
