package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.identifier
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SecureStorageKeyTest {

    @Test
    fun testIdentifier() {
        val factorSourceId = DeviceFactorSource.sample().id
        val key = SecureStorageKey.DeviceFactorSourceMnemonic(factorSourceId = factorSourceId)
        assertEquals(
            "secure_storage_key_device_factor_source_device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a",
            key.identifier
        )

        assertEquals(
            "secure_storage_key_host_id",
            SecureStorageKey.HostId.identifier
        )

        assertEquals(
            "secure_storage_key_profile_snapshot",
            SecureStorageKey.ProfileSnapshot.identifier
        )
    }

}