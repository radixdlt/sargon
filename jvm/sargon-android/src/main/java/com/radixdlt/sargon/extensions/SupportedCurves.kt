package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Slip10Curve

class SupportedCurves private constructor(
    array: IdentifiedArray<Slip10Curve, Slip10Curve>
) : IdentifiedArray<Slip10Curve, Slip10Curve> by array {

    constructor(curves: List<Slip10Curve>) : this(
        IdentifiedArrayImpl(
            elements = curves,
            identifier = { it }
        )
    )

    constructor(vararg curve: Slip10Curve) : this(curves = curve.asList())
}