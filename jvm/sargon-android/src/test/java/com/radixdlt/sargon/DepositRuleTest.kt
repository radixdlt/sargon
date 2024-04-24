package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class DepositRuleTest {

    @Test
    fun testJsonRoundtrip() {
        val sut = DepositRule.sample()
        Assertions.assertEquals(
            sut,
            DepositRule.fromJson(sut.toJson())
        )
    }
}