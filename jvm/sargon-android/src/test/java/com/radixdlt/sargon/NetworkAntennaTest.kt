package com.radixdlt.sargon

import com.radixdlt.sargon.antenna.SargonNetworkingDriver
import com.radixdlt.sargon.extensions.compareTo
import com.radixdlt.sargon.extensions.toDecimal192
import com.radixdlt.sargon.extensions.toHttpMethod
import com.radixdlt.sargon.samples.sampleMainnet
import kotlinx.coroutines.runBlocking
import okhttp3.OkHttpClient
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Tag
import org.junit.jupiter.api.Test

class NetworkAntennaTest {

    private val okHttpClient = OkHttpClient.Builder().build()

    @Test
    @Tag("IntegrationTests")
    fun testNetwork() = runBlocking {
        val client = GatewayClient(SargonNetworkingDriver(okHttpClient), NetworkId.MAINNET)

        val xrdBalance = client.xrdBalanceOfAccountOrZero(address = AccountAddress.sampleMainnet())

        assertTrue(xrdBalance >= 1.toDecimal192())
    }

    @Test
    fun testHttpMethod() {
        assertEquals("POST", NetworkMethod.POST.toHttpMethod())
        assertEquals("GET", NetworkMethod.GET.toHttpMethod())
    }

}