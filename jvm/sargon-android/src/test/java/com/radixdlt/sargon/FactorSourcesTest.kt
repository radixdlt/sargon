package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.FactorSources
import com.radixdlt.sargon.extensions.SupportedCurves
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class FactorSourcesTest : IdentifiedArrayTest<FactorSources, FactorSourceId, FactorSource>() {

    override fun element(): FactorSource = FactorSource.sample()

    override fun elementWithDifferentId(): FactorSource = FactorSource.sample.other()

    override fun identifier(element: FactorSource): FactorSourceId = element.id

    override fun init(element: FactorSource): FactorSources = FactorSources(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            FactorSources(
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

    @Test
    fun testDeviceFactorSourceAsGeneral() {
        val factorSource = DeviceFactorSource.sample.other()

        assertEquals(
            FactorSource.Device(factorSource),
            factorSource.asGeneral()
        )
    }

    @Test
    fun testLedgerFactorSourceAsGeneral() {
        val factorSource = LedgerHardwareWalletFactorSource.sample.other()

        assertEquals(
            FactorSource.Ledger(factorSource),
            factorSource.asGeneral()
        )
    }

}