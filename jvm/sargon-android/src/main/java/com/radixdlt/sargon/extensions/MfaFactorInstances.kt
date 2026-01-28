package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorInstance
import com.radixdlt.sargon.MfaFactorInstance
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.mFAFactorInstanceToJsonBytes
import com.radixdlt.sargon.mfaFactorInstancesToJsonBytes
import com.radixdlt.sargon.newMFAFactorInstanceFromJsonBytes
import com.radixdlt.sargon.newMfaFactorInstancesFromJsonBytes
import com.radixdlt.sargon.newP2pLinksFromJsonBytes
import com.radixdlt.sargon.p2pLinksToJsonBytes

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

@Throws(SargonException::class)
fun MfaFactorInstances.Companion.fromJson(json: String) =
    newMfaFactorInstancesFromJsonBytes(jsonBytes = bagOfBytes(json))

fun MfaFactorInstances.toJson(): String =
    mfaFactorInstancesToJsonBytes(asList()).string