package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleGlobalIdTest {

    @Test
    fun test() {
        val expected = NonFungibleGlobalId(
            resourceAddress = ResourceAddress.init(
                NonFungibleResourceAddress.sampleMainnet().string
            ),
            nonFungibleLocalId = NonFungibleLocalId.init("#1#"),
            asString = "",
            formatted = FormattedAddress("", "", "")
        )

        assertEquals(
            expected,
            NonFungibleGlobalId.init(
                "${NonFungibleResourceAddress.sampleMainnet().string}:#1#"
            )
        )
    }

    @Test
    fun testRoundtrip() {
        assertEquals(
            NonFungibleGlobalId.sample(),
            NonFungibleGlobalId.init(globalId = NonFungibleGlobalId.sample().string)
        )
    }

    @Test
    fun testFormat() {
        val address = NonFungibleGlobalId(
            resourceAddress = ResourceAddress.init(
                NonFungibleResourceAddress.sampleMainnet().string
            ),
            nonFungibleLocalId = NonFungibleLocalId.init("#1#"),
            asString = "",
            formatted = FormattedAddress("", "", "")
        )

        assertEquals("reso...c9wlxa:#1#", address.formatted())
        assertEquals(
            "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:1",
            address.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            address.string,
            address.formatted(format = AddressFormat.RAW)
        )
    }

}