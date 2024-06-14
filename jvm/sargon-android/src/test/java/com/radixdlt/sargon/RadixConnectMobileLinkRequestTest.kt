package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class RadixConnectMobileLinkRequestTest: SampleTestable<RadixConnectMobileLinkRequest> {
    override val samples: List<Sample<RadixConnectMobileLinkRequest>>
        get() = listOf(RadixConnectMobileLinkRequest.sample)

    @Test
    fun testJsonRoundtrip() {
        assertEquals(
            RadixConnectMobileLinkRequest.sample(),
            RadixConnectMobileLinkRequest.fromJson(RadixConnectMobileLinkRequest.sample().toJson())
        )
    }

}