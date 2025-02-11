package com.radixdlt.sargon.os.storage.key

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.hostIdToJsonBytes
import com.radixdlt.sargon.newHostIdFromJsonBytes
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.samples.sample
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.io.File

class HostIdKeyMappingTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    private val mnemonicWithPassphrase = MnemonicWithPassphrase.sample()

    @field:TempDir
    lateinit var tmpDir: File

    private val storage = PreferenceDataStoreFactory.create(scope = testScope) {
        File(tmpDir, "test.preferences_pb")
    }

    @Test
    fun testRoundtrip() = runTest(context = testDispatcher) {
        val hostId = HostId.sample()

        val sut = HostIdKeyMapping(
            key = SecureStorageKey.HostId,
            deviceStorage = storage
        )

        val writeResult = sut.write(hostIdToJsonBytes(hostId))
        assertTrue(writeResult.isSuccess)

        val readResult = sut.read()
        assertEquals(
            hostId,
            newHostIdFromJsonBytes(readResult.getOrThrow()!!)
        )

        // Test the key is the correct one by testing a read directly through storage
        val readDirectlyFromStorageResult = storage.read(
            key = stringPreferencesKey("key_device_info")
        )
        assertEquals(
            HostIdAndroidEntry(
                id = hostId.id,
                date = hostId.generatedAt
            ),
            HostIdAndroidEntry.fromJsonString(readDirectlyFromStorageResult.getOrThrow()!!)
        )
        assertEquals(
            hostId,
            // Also test the dance between android specific mapping and sargon mapping are
            // producing the expected result
            HostIdAndroidEntry.fromJsonString(readDirectlyFromStorageResult.getOrThrow()!!).toHostId()
        )

        val removeResult = sut.remove()
        assertTrue(removeResult.isSuccess)
        val readResultAfterRemove = sut.read()
        assertNull(readResultAfterRemove.getOrThrow())
        assertFalse(sut.keyExist())
    }

}