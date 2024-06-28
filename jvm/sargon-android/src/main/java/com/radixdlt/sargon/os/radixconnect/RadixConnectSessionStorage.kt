@file:OptIn(ExperimentalUnsignedTypes::class)

package com.radixdlt.sargon.os.radixconnect

import android.content.Context
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.byteArrayPreferencesKey
import androidx.datastore.preferences.preferencesDataStoreFile
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.SessionId
import com.radixdlt.sargon.RadixConnectMobileSessionStorage
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.os.storage.EncryptedPreferencesStorage
import com.radixdlt.sargon.os.storage.KeySpec

internal class RadixConnectSessionStorage internal constructor(
    private val storage: EncryptedPreferencesStorage
) : RadixConnectMobileSessionStorage {

    @KoverIgnore
    constructor(context: Context) : this(
        storage = EncryptedPreferencesStorage(datastore = PreferenceDataStoreFactory.create() {
            val applicationContext = context.applicationContext
            applicationContext.preferencesDataStoreFile(STORAGE_FILE_NAME)
        })
    )

    override suspend fun saveSession(sessionId: SessionId, encodedSession: BagOfBytes) {
        storage.set(
            sessionId.key(),
            encodedSession.toUByteArray().toByteArray(),
            KeySpec.RadixConnect()
        )
    }

    override suspend fun loadSession(sessionId: SessionId): BagOfBytes? = storage.get(
        key = sessionId.key(),
        keySpec = KeySpec.RadixConnect()
    ).getOrNull()?.toBagOfBytes()

    private fun SessionId.key() = byteArrayPreferencesKey(name = toString())

    companion object {
        private const val STORAGE_FILE_NAME = "rdx_radix_connect_session_storage"
    }
}