package com.radixdlt.sargon.os.storage.key

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.newVecOfFactorSourceIdFromHashFromJson
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.vecOfFactorSourceIdFromHashToJson
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

class FactorSourceUserHasWrittenDownKeyMappingTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    @field:TempDir
    lateinit var tmpDir: File

    private val storage = PreferenceDataStoreFactory.create(scope = testScope) {
        File(tmpDir, "test.preferences_pb")
    }

    @Test
    fun testEmpty() = runTest(context = testDispatcher) {
        val sut = FactorSourceUserHasWrittenDownKeyMapping(
            storage = storage
        )

        val value = sut.read().getOrThrow()

        assertNull(value)
    }

    @Test
    fun testReadOneFromAndroidStorageUsedInSargon() = runTest(context = testDispatcher) {
        val hex1 = Exactly32Bytes.sample().hex

        storage.edit { preferences ->
            preferences[stringPreferencesKey("backed_up_factor_source_ids")] = hex1
        }

        val sut = FactorSourceUserHasWrittenDownKeyMapping(
            storage = storage
        )

        val idsBytes = requireNotNull(sut.read().getOrThrow())
        val ids = newVecOfFactorSourceIdFromHashFromJson(idsBytes)

        assertEquals(1, ids.size)
        assertEquals(hex1, ids[0].body.hex)
    }

    @Test
    fun testReadMultipleFromAndroidStorageUsedInSargon() = runTest(context = testDispatcher) {
        val hex1 = Exactly32Bytes.sample().hex
        val hex2 = Exactly32Bytes.sample.other().hex

        storage.edit { preferences ->
            preferences[stringPreferencesKey("backed_up_factor_source_ids")] = "$hex1,$hex2"
        }

        val sut = FactorSourceUserHasWrittenDownKeyMapping(
            storage = storage
        )

        val idsBytes = requireNotNull(sut.read().getOrThrow())
        val ids = newVecOfFactorSourceIdFromHashFromJson(idsBytes)

        assertEquals(2, ids.size)
        assertEquals(hex1, ids[0].body.hex)
        assertEquals(hex2, ids[1].body.hex)
    }

    @Test
    fun testRoundtrip() = runTest(context = testDispatcher) {
        val factorSourceIds = listOf(
            FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.sample()
            ),
            FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.sample.other()
            )
        )
        // Serialize those ids as sargon would
        val bagOfBytes = vecOfFactorSourceIdFromHashToJson(ids = factorSourceIds)

        val sut = FactorSourceUserHasWrittenDownKeyMapping(
            storage = storage
        )

        // Write those into the android storage
        assert(sut.write(bagOfBytes).isSuccess)
        // Check if key exists after writing
        assertTrue(sut.keyExist())

        // Read from android storage
        val value = requireNotNull(sut.read().getOrThrow())
        // Deserialize them as sargon would
        val result = newVecOfFactorSourceIdFromHashFromJson(value)

        assertEquals(
            factorSourceIds,
            result
        )
    }

    @Test
    fun testRemove() = runTest(context = testDispatcher) {
        val factorSourceIds = listOf(
            FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.sample()
            ),
            FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.sample.other()
            )
        )
        // Serialize those ids as sargon would
        val bagOfBytes = vecOfFactorSourceIdFromHashToJson(ids = factorSourceIds)

        val sut = FactorSourceUserHasWrittenDownKeyMapping(
            storage = storage
        )

        // Write those into the android storage
        assert(sut.write(bagOfBytes).isSuccess)

        // Remove from storage
        assert(sut.remove().isSuccess)

        assertFalse(sut.keyExist())
        assertNull(sut.read().getOrThrow())
    }
}