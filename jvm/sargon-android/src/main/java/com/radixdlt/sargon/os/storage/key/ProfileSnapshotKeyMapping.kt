package com.radixdlt.sargon.os.storage.key

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest
import com.radixdlt.sargon.os.storage.keyExist
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.remove
import com.radixdlt.sargon.os.storage.write
import kotlinx.coroutines.delay
import java.io.IOException

internal class ProfileSnapshotKeyMapping(
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
            keystoreAccessRequest = KeystoreAccessRequest.ForProfile
        )
    }

    override suspend fun read(): Result<BagOfBytes?> = encryptedStorage.read(
        key = preferenceKey,
        keystoreAccessRequest = KeystoreAccessRequest.ForProfile,
        retryWhen = { cause, attempt ->
            // It seems that some users are experiencing IOException which results in empty preferences returned.
            // We retry to read preferences if IOException occurred, before we go to catchIOException() handler.
            // This code was directly taken from https://github.com/radixdlt/babylon-wallet-android/blob/8de8e5016f4261ae3679d67faf3b474f88c5b691/profile/src/main/java/rdx/works/profile/datastore/EncryptedPreferencesManager.kt#L38
            if (cause is IOException && attempt < RETRY_ON_IO_EXCEPTION_COUNT) {
                delay(RETRY_DELAY)
                true
            } else {
                false
            }
        }
    ).mapCatching { snapshotString ->
        snapshotString?.let { bagOfBytes(snapshotString) }
    }

    override suspend fun remove(): Result<Unit> = encryptedStorage.remove(preferenceKey)

    override suspend fun keyExist(): Boolean = encryptedStorage.keyExist(preferenceKey)


    companion object {
        private const val KEY = "profile_preferences_key"
        private const val RETRY_ON_IO_EXCEPTION_COUNT = 3L
        private const val RETRY_DELAY = 1500L
    }
}
