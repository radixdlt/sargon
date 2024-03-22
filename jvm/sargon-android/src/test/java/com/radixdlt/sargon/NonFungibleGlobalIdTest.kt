package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.from
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
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
            nonFungibleLocalId = NonFungibleLocalId.from("#1#")
        )

        assertEquals(
            expected,
            NonFungibleGlobalId.from(
                "${NonFungibleResourceAddress.sampleMainnet().string}:#1#"
            )
        )
    }

}