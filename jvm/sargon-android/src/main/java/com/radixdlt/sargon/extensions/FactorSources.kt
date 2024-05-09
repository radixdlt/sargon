package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId

class FactorSources private constructor(
    array: IdentifiedArray<FactorSourceId, FactorSource>
) : IdentifiedArray<FactorSourceId, FactorSource> by array {

    constructor(factorSources: List<FactorSource>) : this(
        IdentifiedArrayImpl(
            elements = factorSources,
            identifier = { it.id }
        )
    )

    constructor(vararg factorSource: FactorSource) : this(
        factorSources = factorSource.asList()
    )
}

