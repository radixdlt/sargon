package com.radixdlt.sargon.os.radixconnect

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.byteArrayPreferencesKey
import androidx.datastore.preferences.preferencesDataStoreFile
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.RadixConnectMobileSessionStorage
import com.radixdlt.sargon.SessionId
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.write

internal class RadixConnectSessionStorage internal constructor(
    private val dataStore: DataStore<Preferences>
) : RadixConnectMobileSessionStorage {

    @KoverIgnore
    constructor(context: Context) : this(
        dataStore = PreferenceDataStoreFactory.create {
            val applicationContext = context.applicationContext
            applicationContext.preferencesDataStoreFile(STORAGE_FILE_NAME)
        }
    )

    override suspend fun saveSession(sessionId: SessionId, encodedSession: BagOfBytes) {
        dataStore.write(
            key = sessionId.key(),
            value = encodedSession.toByteArray(),
            keystoreAccessRequest = KeystoreAccessRequest.ForRadixConnect
        )
    }

    override suspend fun loadSession(sessionId: SessionId): BagOfBytes? = dataStore.read(
        key = sessionId.key(),
        keystoreAccessRequest = KeystoreAccessRequest.ForRadixConnect
    ).getOrNull()?.toBagOfBytes()

    private fun SessionId.key() = byteArrayPreferencesKey(name = toString())

    companion object {
        private const val STORAGE_FILE_NAME = "rdx_radix_connect_session_storage"
    }
}