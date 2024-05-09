package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.annotation.KoverIgnore

class FactorSources private constructor(
    private val array: IdentifiedArray<FactorSourceId, FactorSource>
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

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as FactorSources

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "FactorSources(array=$array)"
    }

}

fun List<FactorSource>.asIdentifiable() = FactorSources(factorSources = this)
