package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class FactorSourceKindTest {

    @Test
    fun testStringRoundtrip() {
        assertEquals(
            FactorSourceKind.DEVICE,
            FactorSourceKind.init(FactorSourceKind.DEVICE.string)
        )
    }

}