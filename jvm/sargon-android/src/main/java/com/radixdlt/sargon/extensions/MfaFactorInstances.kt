package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorInstance
import com.radixdlt.sargon.MfaFactorInstance
import com.radixdlt.sargon.annotation.KoverIgnore

class MfaFactorInstances private constructor(
    private val array: IdentifiedArray<FactorInstance, MfaFactorInstance>
) : IdentifiedArray<FactorInstance, MfaFactorInstance> by array {

    constructor(mfaFactorInstances: List<MfaFactorInstance>) : this(
        IdentifiedArrayImpl(
            elements = mfaFactorInstances,
            identifier = { it.factorInstance }
        )
    )

    constructor(vararg mfaFactorInstance: MfaFactorInstance) : this(mfaFactorInstances = mfaFactorInstance.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as MfaFactorInstances

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "MfaFactorInstances(array=$array)"
    }

    companion object

}

fun List<MfaFactorInstance>.asIdentifiable() = MfaFactorInstances(mfaFactorInstances = this)