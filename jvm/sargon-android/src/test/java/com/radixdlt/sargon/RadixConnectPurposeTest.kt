package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class RadixConnectPurposeTest {
    @Test
    fun testStringRoundtrip() {
        assertEquals(
            RadixConnectPurpose.GENERAL,
            RadixConnectPurpose.init("general")
        )
    }
}