package com.radixdlt.sargon.os.driver

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AndroidEntropyProviderDriverTest {

    private val sut = AndroidEntropyProviderDriver()

    @Test
    fun testEntropy() {
        val cases = 1000
        assertEquals(
            cases,
            List(cases) {
                sut.generateSecureRandomBytes()
            }.toSet().size
        )
    }
}