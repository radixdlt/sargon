package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.SupportedCurves
import com.radixdlt.sargon.newSupportedCurves
import com.radixdlt.sargon.newSupportedCurvesByAppending
import com.radixdlt.sargon.newSupportedCurvesByUpdatingOrAppending
import com.radixdlt.sargon.newSupportedCurvesByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newSupportedCurvesRemovedElement
import com.radixdlt.sargon.supportedCurvesElementCount
import com.radixdlt.sargon.supportedCurvesGetElements

fun SupportedCurves.Companion.init(vararg curve: Slip10Curve): SupportedCurves =
    init(curves = curve.asList())

fun SupportedCurves.Companion.init(curves: List<Slip10Curve>): SupportedCurves =
    newSupportedCurves(supportedCurves = curves)

operator fun SupportedCurves.invoke() = supportedCurvesGetElements(supportedCurves = this)

operator fun SupportedCurves.get(index: Int) = invoke().get(index = index)

operator fun SupportedCurves.contains(element: Slip10Curve) = invoke().contains(element = element)

val SupportedCurves.size: Int
    get() = supportedCurvesElementCount(supportedCurves = this).toInt()

fun SupportedCurves.append(curve: Slip10Curve): SupportedCurves =
    newSupportedCurvesByAppending(sLIP10Curve = curve, to = this)

fun SupportedCurves.updateOrInsert(
    curve: Slip10Curve,
    index: Int
): SupportedCurves = newSupportedCurvesByUpdatingOrInsertingAtIndex(
    sLIP10Curve = curve,
    to = this,
    index = index.toULong()
)

fun SupportedCurves.updateOrAppend(curve: Slip10Curve): SupportedCurves =
    newSupportedCurvesByUpdatingOrAppending(sLIP10Curve = curve, to = this)

fun SupportedCurves.remove(curve: Slip10Curve): SupportedCurves =
    newSupportedCurvesRemovedElement(sLIP10Curve = curve, from = this)