@file:OptIn(ExperimentalUnsignedTypes::class)

package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.NetworkRequest
import com.radixdlt.sargon.NetworkResponse
import com.radixdlt.sargon.NetworkingDriver
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.extensions.toHttpMethod
import kotlinx.coroutines.ExperimentalCoroutinesApi
import okhttp3.Headers.Companion.toHeaders
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Response
import okhttp3.coroutines.executeAsync

class AndroidNetworkingDriver(
    private val client: OkHttpClient
) : NetworkingDriver {
    @OptIn(ExperimentalCoroutinesApi::class)
    override suspend fun executeNetworkRequest(
        request: NetworkRequest
    ): NetworkResponse = runCatching {
        val mediaType = request.headers.extractMediaType()

        val requestBody = request.body
            .toUByteArray()
            .toByteArray()
            .toRequestBody(contentType = mediaType)

        val okHttpRequest = Request.Builder()
            .url(url = request.url)
            .headers(request.headers.toHeaders())
            .method(method = request.method.toHttpMethod(), body = requestBody)
            .build()

        client.newCall(okHttpRequest).executeAsync()
    }.toNetworkResponse()

    @KoverIgnore
    private fun Result<Response>.toNetworkResponse() = fold(
        onSuccess = { response ->
            NetworkResponse(
                statusCode = response.code.toUShort(),
                body = response.body.bytes().toBagOfBytes()
            )
        },
        onFailure = {
            throw CommonException.NetworkRequestGenericFailure(underlying = it.message.orEmpty())
        }
    )

    @KoverIgnore
    private fun Map<String, String>.extractMediaType() =
        (this["Content-Type"] ?: "application/json; charset=utf-8").toMediaType()
}