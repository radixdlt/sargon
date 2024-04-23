package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.newSLIP10CurveFromJsonString
import com.radixdlt.sargon.newSlip10CurveFromString
import com.radixdlt.sargon.sLIP10CurveToJsonString
import com.radixdlt.sargon.slip10CurveToString

@Throws(SargonException::class)
fun Slip10Curve.Companion.init(value: String) = newSlip10CurveFromString(curve = value)

val Slip10Curve.string: String
    get() = slip10CurveToString(curve = this)

@Throws(SargonException::class)
fun Slip10Curve.Companion.deserializeFromJsonString(jsonString: String) =
    newSLIP10CurveFromJsonString(jsonString = jsonString)

fun Slip10Curve.serializedJsonString() = sLIP10CurveToJsonString(sLIP10Curve = this)

