package com.radixdlt.sargon

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
            nonFungibleLocalId = NonFungibleLocalId.init("#1#")
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

}