package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Gateways
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

internal class GatewaysTest: IdentifiedArrayTest<Gateways, Url, Gateway>() {
    override fun element(): Gateway = Gateway.sampleMainnet()

    override fun elementWithDifferentId(): Gateway = Gateway.sampleStokenet()

    override fun identifier(element: Gateway): Url = element.url

    override fun init(element: Gateway): Gateways = Gateways(element)

    @Test
    fun similarUrlsTest() {
        val mainnet = Gateway(
            network = NetworkDefinition(
                logicalName = "mainnet",
                id = NetworkId.MAINNET,
                displayDescription = "mainnet"
            ),
            url = Url("https://mainnet.radixdlt.com")
        )

        val mainnetWithSlash = mainnet.copy(
            url = Url("https://mainnet.radixdlt.com/")
        )

        val otherMainnet = mainnet.copy(
            url = Url("https://mainnet-other.radixdlt.com/")
        )

        assertEquals(
            listOf(mainnet, otherMainnet),
            Gateways(mainnet, mainnetWithSlash, otherMainnet).asList()
        )
    }

}