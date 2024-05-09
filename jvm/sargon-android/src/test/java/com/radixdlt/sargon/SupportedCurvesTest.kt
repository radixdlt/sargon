package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.SupportedCurves
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class SupportedCurvesTest: IdentifiedArrayTest<SupportedCurves, Slip10Curve, Slip10Curve>() {

    override fun element(): Slip10Curve = Slip10Curve.sample()

    override fun elementWithDifferentId(): Slip10Curve = Slip10Curve.sample.other()

    override fun identifier(element: Slip10Curve): Slip10Curve = element

    override fun init(element: Slip10Curve): SupportedCurves = SupportedCurves(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            SupportedCurves(
                element(),
                elementWithDifferentId()
            ),
            listOf(
                element(),
                elementWithDifferentId()
            ).asIdentifiable()
        )
    }

    @Test
    fun testEquality() {
        val element = element()

        assertEquals(
            listOf(element).asIdentifiable(),
            listOf(element).asIdentifiable()
        )

        val collection = listOf(element).asIdentifiable()
        assertEquals(collection, collection)
        assertNotEquals(collection, "")
    }

    @Test
    fun testUniqueness() {
        val element = element()
        val elementOther = elementWithDifferentId()
        assertEquals(
            2,
            setOf(
                listOf(element).asIdentifiable(),
                listOf(elementOther).asIdentifiable(),
                listOf(element).asIdentifiable()
            ).size
        )
    }
}