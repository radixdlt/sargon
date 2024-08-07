package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Gateways
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import okhttp3.HttpUrl.Companion.toHttpUrl
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class GatewaysTest: IdentifiedArrayTest<Gateways, Url, Gateway>() {
    override fun element(): Gateway = Gateway.sampleMainnet()

    override fun elementWithDifferentId(): Gateway = Gateway.sampleStokenet()

    override fun identifier(element: Gateway): Url = element.url

    override fun init(element: Gateway): Gateways = Gateways(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            Gateways(
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
    fun similarUrlsTest() {
        val mainnet = Gateway(
            network = NetworkDefinition(
                logicalName = "mainnet",
                id = NetworkId.MAINNET,
                displayDescription = "mainnet"
            ),
            url = "https://mainnet.radixdlt.com".toHttpUrl()
        )

        val mainnetWithSlash = mainnet.copy(
            url = "https://mainnet.radixdlt.com/".toHttpUrl()
        )

        val otherMainnet = mainnet.copy(
            url = "https://mainnet-other.radixdlt.com/".toHttpUrl()
        )

        assertEquals(
            listOf(mainnet, otherMainnet),
            Gateways(mainnet, mainnetWithSlash, otherMainnet).asList()
        )
    }

}