package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.discriminant
import com.radixdlt.sargon.extensions.from
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class NetworkIdTest {

    @Test
    fun test() {
        assertEquals(
            NetworkId.MAINNET,
            NetworkId.from(discriminant = 1.toUByte())
        )
        assertThrows<CommonException.UnknownNetworkId> {
            NetworkId.from(discriminant = UByte.MAX_VALUE)
        }
        assertEquals(
            "mainnet",
            NetworkId.MAINNET.string
        )
        assertEquals(
            1.toUByte(),
            NetworkId.MAINNET.discriminant
        )
    }

}