package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AssetException
import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl
import com.radixdlt.sargon.ResourceAddress

class AssetsExceptionList private constructor(
    array: IdentifiedArray<ResourceAddress, AssetException>
) : IdentifiedArray<ResourceAddress, AssetException> by array {

    constructor(assetExceptions: List<AssetException>) : this(
        IdentifiedArrayImpl(
            elements = assetExceptions,
            identifier = { it.address }
        )
    )

    constructor(
        vararg assetException: AssetException
    ) : this(assetExceptions = assetException.asList())
}