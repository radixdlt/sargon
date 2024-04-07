package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.errorCode
import com.radixdlt.sargon.extensions.errorMessage
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SargonExceptionTest {

    @Test
    fun testErrorCode() {
        assertEquals(10049u, CommonException.UnknownNetworkForId(badValue = 99u).errorCode)
    }

    @Test
    fun testErrorMessage() {
        assertEquals(
            "No network found with id: '99'",
            CommonException.UnknownNetworkForId(badValue = 99u).errorMessage
        )
    }
}