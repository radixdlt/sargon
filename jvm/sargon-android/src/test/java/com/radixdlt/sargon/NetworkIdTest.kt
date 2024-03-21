package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.discriminant
import com.radixdlt.sargon.extensions.from
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class NetworkIdTest: SampleTestable<NetworkId> {

    override val samples: List<Sample<NetworkId>>
        get() = listOf(NetworkId.sample)

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