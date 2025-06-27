package com.radixdlt.sargon.os.storage.key

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.Timestamp
import com.radixdlt.sargon.Uuid
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.hostIdToJsonBytes
import com.radixdlt.sargon.newHostIdFromJsonBytes
import com.radixdlt.sargon.os.storage.keyExist
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.remove
import com.radixdlt.sargon.os.storage.write
import com.radixdlt.sargon.serializer.TimestampSerializer
import com.radixdlt.sargon.serializer.UuidSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

internal class HostIdKeyMapping(
    private val key: SecureStorageKey,
    private val deviceStorage: DataStore<Preferences>
): DatastoreKeyMapping {

    private val preferencesKey = stringPreferencesKey(PREFERENCES_KEY)

    private fun HostId.Companion.fromJsonBytes(jsonBytes: BagOfBytes) =
        newHostIdFromJsonBytes(jsonBytes)
    private fun HostId.toJsonBytes() = hostIdToJsonBytes(hostId = this)
    private fun HostId.asEntry() = HostIdAndroidEntry(
        id = id,
        date = generatedAt
    )

    override suspend fun write(bagOfBytes: BagOfBytes): Result<Unit> = runCatching {
        HostId.fromJsonBytes(bagOfBytes).asEntry().toJsonString()
    }.then { json ->
        deviceStorage.write(preferencesKey, json)
    }

    override suspend fun read(): Result<BagOfBytes?> = deviceStorage.read(preferencesKey)
        .mapCatching { entrySerialized ->
            if (entrySerialized != null) {
                HostIdAndroidEntry.fromJsonString(entrySerialized).toHostId().toJsonBytes()
            } else {
                null
            }
        }

    override suspend fun remove(): Result<Unit> = deviceStorage.remove(preferencesKey)

    override suspend fun keyExist(): Boolean = deviceStorage.keyExist(preferencesKey)

    companion object {
        private const val PREFERENCES_KEY = "key_device_info"
    }
}

/**
 * Entry stored on device preferences. Kept to ensure compatibility with previous versions
 *
 * - In version 1.6.0 DeviceInfo object was introduced in preferences with
 *  -- id: Uuid,
 *  -- date: Timestamp,
 *  -- name: String,
 *  -- manufacturer: String,
 *  -- model: String
 *  the intention was to keep a stable identifier along with some more data.
 *
 *  - From version 1.8.3 there is no need to keep the all the rest of the data in the preferences.
 * The update was to bridge compatibility with sargon os and android implementation
 *
 * Only the id and date are kept and the rest of the values are not needed, since [HostInfo] will
 * be calculated on the fly by sargon os.
 * So [HostIdAndroidEntry] contains actually a subset of critical data being kept in DeviceInfo previously.
 */
@Serializable
data class HostIdAndroidEntry(
    @Serializable(with = UuidSerializer::class)
    private val id: Uuid,
    @Serializable(with = TimestampSerializer::class)
    private val date: Timestamp
) {

    fun toHostId() = HostId(
        id = id,
        generatedAt = date
    )

    fun toJsonString(): String = Json.encodeToString(this)

    companion object {
        private val jsonSerializer: Json
            get() = Json {
                // Ignore previous values for compatibility
                ignoreUnknownKeys = true
                isLenient = true
            }

        fun fromJsonString(jsonString: String) = jsonSerializer
            .decodeFromString<HostIdAndroidEntry>(jsonString)
    }
}