package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.SupportedCurves
import com.radixdlt.sargon.samples.sample

internal class SupportedCurvesTest: IdentifiedArrayTest<SupportedCurves, Slip10Curve, Slip10Curve>() {

    override fun element(): Slip10Curve = Slip10Curve.sample()

    override fun elementWithDifferentId(): Slip10Curve = Slip10Curve.sample.other()

    override fun identifier(element: Slip10Curve): Slip10Curve = element

    override fun init(element: Slip10Curve): SupportedCurves = SupportedCurves(element)
}