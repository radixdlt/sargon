package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.newSlip10CurveFromString
import com.radixdlt.sargon.slip10CurveToString

@Throws(SargonException::class)
fun Slip10Curve.Companion.init(value: String) = newSlip10CurveFromString(curve = value)

val Slip10Curve.string: String
    get() = slip10CurveToString(curve = this)