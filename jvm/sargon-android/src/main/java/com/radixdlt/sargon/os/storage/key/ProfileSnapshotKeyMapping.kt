package com.radixdlt.sargon.os.storage.key

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.os.storage.KeySpec
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.remove
import com.radixdlt.sargon.os.storage.write

internal class ProfileSnapshotKeyMapping(
    private val key: SecureStorageKey.ProfileSnapshot,
    private val encryptedStorage: DataStore<Preferences>
) : DatastoreKeyMapping {

    private val preferenceKey = stringPreferencesKey(KEY)

    override suspend fun write(
        bagOfBytes: BagOfBytes
    ): Result<Unit> = runCatching {
        bagOfBytes.string
    }.then { snapshotString ->
        encryptedStorage.write(
            key = preferenceKey,
            value = snapshotString,
            keySpec = KeySpec.Profile()
        )
    }

    override suspend fun read(): Result<BagOfBytes?> = encryptedStorage.read(
        key = preferenceKey,
        keySpec = KeySpec.Profile()
    ).mapCatching { snapshotString ->
        snapshotString?.let { bagOfBytes(snapshotString) }
    }

    override suspend fun remove(): Result<Unit> = encryptedStorage.remove(preferenceKey)

    companion object {
        private const val KEY = "profile_preferences_key"
    }
}
