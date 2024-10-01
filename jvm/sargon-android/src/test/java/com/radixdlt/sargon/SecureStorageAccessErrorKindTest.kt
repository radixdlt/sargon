package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.isManualCancellation
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SecureStorageAccessErrorKindTest {

    @Test
    fun testIsManualCancellation() {
        SecureStorageAccessErrorKind.entries.forEach {
            assertEquals(
                it.isManualCancellation(),
                secureStorageAccessErrorKindIsManualCancellation(it)
            )
        }
    }

}