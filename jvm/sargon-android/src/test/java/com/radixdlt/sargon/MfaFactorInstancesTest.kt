package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.MfaFactorInstances
import com.radixdlt.sargon.extensions.P2pLinks
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class MfaFactorInstancesTest : IdentifiedArrayTest<MfaFactorInstances, FactorInstance, MfaFactorInstance>() {

    override fun element(): MfaFactorInstance = MfaFactorInstance.sample()

    override fun elementWithDifferentId(): MfaFactorInstance = MfaFactorInstance.sample.other()

    override fun identifier(element: MfaFactorInstance): FactorInstance = element.factorInstance

    override fun init(element: MfaFactorInstance): MfaFactorInstances = MfaFactorInstances(element)

    @Test
    fun testAsIdentifiable() {
        val element = element()
        val elementOther = elementWithDifferentId()
        assertEquals(
            MfaFactorInstances(
                element,
                elementOther
            ),
            listOf(
                element,
                elementOther
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

    @Test
    fun testJsonRoundtrip() {
        val sut = listOf(element(), elementWithDifferentId())

        assertEquals(
            sut,
            MfaFactorInstances.fromJson(json = sut.asIdentifiable().toJson())
        )
    }
}