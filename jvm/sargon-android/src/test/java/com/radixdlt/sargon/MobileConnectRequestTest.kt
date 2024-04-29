package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.SargonException
import com.radixdlt.sargon.extensions.parseFrom
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class MobileConnectRequestTest {

    @Test
    fun `test link url`() {
        val url = "https://d1rxdfxrfmemlj.cloudfront.net/?sessionId=${Uuid.randomUUID()}&origin=radix%3A%2F%2Fapp"
        val request = MobileConnectRequest.parseFrom(url)
        assertTrue { request is MobileConnectRequest.Link }
    }

    @Test
    fun `test dApp interaction url`() {
        val url = "https://d1rxdfxrfmemlj.cloudfront.net/?sessionId=${Uuid.randomUUID()}&interactionId=${Uuid.randomUUID()}"
        val request = MobileConnectRequest.parseFrom(url)
        assertTrue { request is MobileConnectRequest.DappInteraction }
    }

    @Test
    fun `test exception thrown if url parsed incorrectl`() {
        val url = "https://random.url.net/?randomParam=123"
        assertThrows<SargonException> {
            MobileConnectRequest.parseFrom(url)
        }
    }

}