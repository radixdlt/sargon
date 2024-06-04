package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class WalletToDappInteractionResponseTest {

    @Test
    fun testRoundtrip() {
        val sample = newWalletToDappInteractionResponseSample()
        assertEquals(sample, WalletToDappInteractionResponse.Companion.fromJson(sample.toJson()))
    }

}