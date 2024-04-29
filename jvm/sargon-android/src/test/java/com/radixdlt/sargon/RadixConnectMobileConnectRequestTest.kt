package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.SargonException
import com.radixdlt.sargon.extensions.parseFrom
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class RadixConnectMobileConnectRequestTest {

    @Test
    fun `test link url`() {
        val url = "https://d1rxdfxrfmemlj.cloudfront.net/?sessionId=${Uuid.randomUUID()}&origin=radix%3A%2F%2Fapp"
        val request = RadixConnectMobileConnectRequest.parseFrom(url).getOrNull()
        assertTrue { request is RadixConnectMobileConnectRequest.Link }
    }

    @Test
    fun `test dApp interaction url`() {
        val url = "https://d1rxdfxrfmemlj.cloudfront.net/?sessionId=${Uuid.randomUUID()}&interactionId=${Uuid.randomUUID()}"
        val request = RadixConnectMobileConnectRequest.parseFrom(url).getOrNull()
        assertTrue { request is RadixConnectMobileConnectRequest.DappInteraction }
    }

    @Test
    fun `test exception thrown if url parsed incorrectl`() {
        val url = "https://random.url.net/?randomParam=123"
        assertThrows<SargonException> {
            RadixConnectMobileConnectRequest.parseFrom(url).getOrThrow()
        }
    }

}