package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import java.util.UUID

class HeaderTest: SampleTestable<Header> {

    override val samples: List<Sample<Header>>
        get() = listOf(Header.sample)

    @Test
    fun testJsonRoundtrip() {
        val deviceInfo = DeviceInfo(
            id = UUID.randomUUID(),
            date = Timestamp.now(),
            description = "Samsung Galaxy (My Precious)",
            systemVersion = "0.0.1",
            hostAppVersion = "1.6.0",
            hostVendor = "Samsung"
        )
        val header = Header.init(creatingDevice = deviceInfo)

        assertEquals(
            header,
            Header.fromJson(jsonString = header.toJson())
        )
    }

}