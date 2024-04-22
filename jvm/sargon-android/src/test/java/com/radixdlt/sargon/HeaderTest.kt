package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.deserializeFromString
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.serializedString
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
            description = "Unit test"
        )
        val header = Header.init(creatingDevice = deviceInfo)

        assertEquals(
            header,
            Header.deserializeFromString(jsonString = header.serializedString())
        )
    }

}