package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.identifier
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class UnsafeStorageKeyTest {

    @Test
    fun testIdentifier() {
        assertEquals(
            "unsafe_storage_key_factor_source_user_has_written_down",
            UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN.identifier
        )
    }

}