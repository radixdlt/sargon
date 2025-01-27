package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.values
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class TimePeriodUnitTest {

    @Test
    fun test() {
        assertEquals(TimePeriodUnit.DAYS.values, (1..9999).toList())
    }
}