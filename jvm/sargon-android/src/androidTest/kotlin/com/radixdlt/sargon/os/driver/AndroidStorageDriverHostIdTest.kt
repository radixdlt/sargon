package com.radixdlt.sargon.os.driver

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SmallTest
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.Timestamp
import com.radixdlt.sargon.Uuid
import com.radixdlt.sargon.hostIdToJsonBytes
import com.radixdlt.sargon.newHostIdFromJsonBytes
import com.radixdlt.sargon.os.driver.AndroidStorageDriverTest.Companion.OLD_DEVICE_INFO_PREFERENCES
import com.radixdlt.sargon.os.driver.AndroidStorageDriverTest.Companion.sut
import com.radixdlt.sargon.os.storage.key.HostIdAndroidEntry
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.serializer.TimestampSerializer
import com.radixdlt.sargon.serializer.UuidSerializer
import junit.framework.TestCase.assertEquals
import junit.framework.TestCase.assertNull
import kotlinx.coroutines.test.runTest
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.junit.After
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File
import java.util.UUID

@RunWith(AndroidJUnit4::class)
@SmallTest
class AndroidStorageDriverHostIdTest {

    private val testContext: Context = ApplicationProvider.getApplicationContext()

    @After
    fun deleteDatastores() {
        File(testContext.filesDir, "datastore").deleteRecursively()
        testContext.getSharedPreferences(
            OLD_DEVICE_INFO_PREFERENCES,
            Context.MODE_PRIVATE
        ).edit().clear().commit()
    }

    @Test
    fun testDeviceInfoMigratedDirectlyToDatastore() = runTest {
        // Setup device id in preferences
        val prefEntry = OldDeviceInfoEntry.random()
        val oldSharedPreferences = testContext.getSharedPreferences(
            OLD_DEVICE_INFO_PREFERENCES,
            Context.MODE_PRIVATE
        )
        // store info in old preferences
        oldSharedPreferences
            .edit()
            .putString("key_device_info", Json.encodeToString(prefEntry))
            .commit()
        // Assert that info is stored in old preferences at this point
        with(
            oldSharedPreferences.getString("key_device_info", null)?.let {
                Json.decodeFromString<OldDeviceInfoEntry>(it)
            }
        ) {
            assertEquals(prefEntry.id, this?.id)
            assertEquals(prefEntry.date.toEpochSecond(), this?.date?.toEpochSecond())
        }

        // start driver which internally invokes the migration to datastore
        val sut = sut(testContext, backgroundScope)

        val hostIdBytes = sut.loadData(SecureStorageKey.HostId)
        val hostId = hostIdBytes?.let { newHostIdFromJsonBytes(jsonBytes = it) }

        assertEquals(prefEntry.id, hostId?.id)
        assertEquals(
            prefEntry.date.toEpochSecond(),
            hostId?.generatedAt?.toEpochSecond()
        )
    }

    @Test
    fun testHostIdMigratedDirectlyToDatastore() = runTest {
        // Setup device id in preferences
        val prefEntry = NewHostIdEntry.random()
        val oldSharedPreferences = testContext.getSharedPreferences(
            OLD_DEVICE_INFO_PREFERENCES,
            Context.MODE_PRIVATE
        )
        // store info in old preferences
        oldSharedPreferences
            .edit()
            .putString("key_device_info", Json.encodeToString(prefEntry))
            .commit()
        // Assert that info is stored in old preferences at this point
        with(
            oldSharedPreferences.getString("key_device_info", null)?.let {
                Json.decodeFromString<NewHostIdEntry>(it)
            }
        ) {
            assertEquals(prefEntry.id, this?.id)
            assertEquals(prefEntry.date.toEpochSecond(), this?.date?.toEpochSecond())
        }


        // start driver which internally invokes the migration to datastore
        val sut = sut(testContext, backgroundScope)

        val hostIdBytes = sut.loadData(SecureStorageKey.HostId)
        val hostId = hostIdBytes?.let { newHostIdFromJsonBytes(jsonBytes = it) }

        assertEquals(prefEntry.id, hostId?.id)
        assertEquals(
            prefEntry.date.toEpochSecond(),
            hostId?.generatedAt?.toEpochSecond()
        )
    }

    @Test
    fun testEmptyDeviceInfoPrefsMigratedDirectlyToDatastoreReturnsNull() = runTest {
        // start driver which internally invokes the migration to datastore
        val sut = sut(testContext, backgroundScope)

        val hostIdBytes = sut.loadData(SecureStorageKey.HostId)
        val hostId = hostIdBytes?.let { newHostIdFromJsonBytes(jsonBytes = it) }

        assertNull(hostId)
    }

    @Test
    fun testHostIdCleared() = runTest {
        // start driver which internally invokes the migration to datastore
        val sut = sut(testContext, backgroundScope)

        val json = hostIdToJsonBytes(HostId.sample())
        sut.saveData(SecureStorageKey.HostId, json)
        assertEquals(
            json,
            sut.loadData(SecureStorageKey.HostId)
        )

        sut.deleteDataForKey(SecureStorageKey.HostId)

        assertNull(sut.loadData(SecureStorageKey.HostId))
    }

    // Newer data type stored into preferences or datastore (if already migrated)
    @Serializable
    private data class NewHostIdEntry(
        @Serializable(with = UuidSerializer::class)
        val id: Uuid,
        @Serializable(with = TimestampSerializer::class)
        val date: Timestamp
    ) {

        companion object {
            fun random() = HostIdAndroidEntry(
                id = UUID.randomUUID(),
                date = Timestamp.now()
            )
        }

    }

    // The old data type stored into preferences
    @Serializable
    private data class OldDeviceInfoEntry(
        @Serializable(with = UuidSerializer::class)
        val id: Uuid,
        @Serializable(with = TimestampSerializer::class)
        val date: Timestamp,
        val name: String,
        val manufacturer: String,
        val model: String
    ) {

        companion object {
            fun random() = OldDeviceInfoEntry(
                id = UUID.randomUUID(),
                date = Timestamp.now(),
                name = "Unit",
                manufacturer = "-",
                model = "Test"
            )
        }

    }

}