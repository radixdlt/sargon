package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.annotation.KoverIgnore

class SupportedCurves private constructor(
    private val array: IdentifiedArray<Slip10Curve, Slip10Curve>
) : IdentifiedArray<Slip10Curve, Slip10Curve> by array {

    constructor(curves: List<Slip10Curve>) : this(
        IdentifiedArrayImpl(
            elements = curves,
            identifier = { it }
        )
    )

    constructor(vararg curve: Slip10Curve) : this(curves = curve.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as SupportedCurves

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "SupportedCurves(array=$array)"
    }

}

fun List<Slip10Curve>.asIdentifiable() = SupportedCurves(curves = this)