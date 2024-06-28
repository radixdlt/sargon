package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class RadixConnectMobileSessionRequestTest : SampleTestable<RadixConnectMobileSessionRequest> {
    override val samples: List<Sample<RadixConnectMobileSessionRequest>>
        get() = listOf(RadixConnectMobileSessionRequest.sample)

    @Test
    fun testJsonRoundtrip() {
        assertEquals(
            RadixConnectMobileSessionRequest.sample(),
            RadixConnectMobileSessionRequest.fromJson(
                RadixConnectMobileSessionRequest.sample().toJson()
            )
        )
    }

}