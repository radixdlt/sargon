package com.radixdlt.sargon.os.storage.key

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.byteArrayPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.extensions.identifier
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import com.radixdlt.sargon.os.storage.KeySpec
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.remove
import com.radixdlt.sargon.os.storage.write

internal class ByteArrayKeyMapping private constructor(
    private val input: ByteArrayKeyMappingInput
) : DatastoreKeyMapping {

    internal constructor(
        key: UnsafeStorageKey,
        storage: DataStore<Preferences>
    ) : this(
        ByteArrayKeyMappingInput.Unsecure(
            key = key,
            storage = storage
        )
    )

    internal constructor(
        key: SecureStorageKey,
        keySpec: KeySpec,
        storage: DataStore<Preferences>
    ) : this(
        ByteArrayKeyMappingInput.Secure(
            key = key,
            keySpec = keySpec,
            storage = storage
        )
    )

    private val preferencesKey = when (input) {
        is ByteArrayKeyMappingInput.Secure -> byteArrayPreferencesKey(input.key.identifier)
        is ByteArrayKeyMappingInput.Unsecure -> byteArrayPreferencesKey(input.key.identifier)
    }

    override suspend fun write(bagOfBytes: BagOfBytes): Result<Unit> = when (input) {
        is ByteArrayKeyMappingInput.Secure -> input.storage.write(
            preferencesKey,
            bagOfBytes.toByteArray(),
            input.keySpec
        )

        is ByteArrayKeyMappingInput.Unsecure -> input.storage.write(
            key = preferencesKey,
            value = bagOfBytes.toByteArray()
        )
    }

    override suspend fun read(): Result<BagOfBytes?> = when (input) {
        is ByteArrayKeyMappingInput.Secure -> input.storage.read(preferencesKey, input.keySpec).map {
            it?.toBagOfBytes()
        }
        is ByteArrayKeyMappingInput.Unsecure -> input.storage.read(preferencesKey).map {
            it?.toBagOfBytes()
        }
    }

    override suspend fun remove(): Result<Unit> = when (input) {
        is ByteArrayKeyMappingInput.Secure -> input.storage.remove(preferencesKey)
        is ByteArrayKeyMappingInput.Unsecure -> input.storage.remove(preferencesKey)
    }

    internal sealed interface ByteArrayKeyMappingInput {
        val storage: DataStore<Preferences>

        data class Unsecure(
            val key: UnsafeStorageKey,
            override val storage: DataStore<Preferences>,
        ) : ByteArrayKeyMappingInput

        data class Secure(
            val key: SecureStorageKey,
            val keySpec: KeySpec,
            override val storage: DataStore<Preferences>,
        ) : ByteArrayKeyMappingInput
    }

}