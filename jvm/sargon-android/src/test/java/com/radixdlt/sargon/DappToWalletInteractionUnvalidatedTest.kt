package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DappToWalletInteractionUnvalidatedTest {

    @Test
    fun testRoundtrip() {
        val sample = newDappToWalletInteractionUnvalidatedSample()
        assertEquals(sample, DappToWalletInteractionUnvalidated.Companion.fromJson(sample.toJson().getOrThrow()).getOrThrow())
    }

}