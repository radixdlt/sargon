package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class LinkConnectionQrDataTest : SampleTestable<LinkConnectionQrData> {

    override val samples: List<Sample<LinkConnectionQrData>>
        get() = listOf(LinkConnectionQrData.sample)

    @Test
    fun testJsonRoundtrip() {
        val sut = LinkConnectionQrData.sample.invoke()
        assertEquals(
            sut,
            LinkConnectionQrData.fromJson(json = sut.toJson())
        )
    }
}