package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.value
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class U31Test {

    @Test
    fun testRoundtrip() {
        assertEquals(
            U31(0u).value,
            0u
        )
    }

}