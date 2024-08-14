package com.radixdlt.sargon.os.driver

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SmallTest
import com.radixdlt.sargon.NetworkMethod
import com.radixdlt.sargon.NetworkRequest
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.toBagOfBytes
import kotlinx.coroutines.test.runTest
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import okhttp3.OkHttpClient
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
@SmallTest
class AndroidNetworkingDriverTest {

    private val httpClient = OkHttpClient()
    private val sut = AndroidNetworkingDriver(client = httpClient)

    @Test
    fun test() = runMockWebServer { server ->
        val requestBody = RequestBody(
            id = 10,
            message = "Hello World!"
        )
        val responseBody = ResponseBody(
            id = requestBody.id,
            message = requestBody.message
        )
        server.enqueue(
            MockResponse()
                .setResponseCode(200)
                .setBody(Json.encodeToString(responseBody))
        )

        val response = sut.executeNetworkRequest(
            NetworkRequest(
                url = server.url("/some/api"),
                method = NetworkMethod.POST,
                headers = mapOf(
                    "Content-Type" to "application/json"
                ),
                body = bagOfBytes(Json.encodeToString(requestBody))
            )
        )

        // Request Assertions
        val request = server.takeRequest()
        assertEquals(
            server.url("/some/api"),
            request.requestUrl
        )
        assertEquals(
            "POST",
            request.method
        )
        assertEquals(
            "application/json",
            request.headers["Content-Type"]
        )
        assertEquals(
            bagOfBytes(Json.encodeToString(requestBody)),
            request.body.readByteArray().toBagOfBytes()
        )

        // Response Assertions
        assertEquals(
            200,
            response.statusCode.toInt()
        )
        assertEquals(
            bagOfBytes(Json.encodeToString(responseBody)),
            response.body
        )
    }

    @Serializable
    data class RequestBody(
        val id: Int,
        val message: String
    )

    @Serializable
    data class ResponseBody(
        val id: Int,
        val message: String
    )

    private fun runMockWebServer(test: suspend (MockWebServer) -> Unit) = runTest {
        val server = MockWebServer().apply { start() }

        test(server)

        server.shutdown()
    }
}