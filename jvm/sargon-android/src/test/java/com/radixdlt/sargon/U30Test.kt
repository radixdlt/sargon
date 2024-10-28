package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.value
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class U30Test {

    @Test
    fun testRoundtrip() {
        assertEquals(
            U30(0u).value,
            0u
        )
    }

}