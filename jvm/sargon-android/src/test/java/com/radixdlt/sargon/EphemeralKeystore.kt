package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.identifier

internal object EphemeralKeystore : SecureStorage {
    private val storage: MutableMap<String, ByteArray> = mutableMapOf()

    override fun loadData(key: SecureStorageKey): ByteArray? = storage[key.identifier]

    override fun saveData(key: SecureStorageKey, data: ByteArray) {
        storage[key.identifier] = data
    }

    override fun deleteDataForKey(key: SecureStorageKey) {
        storage.remove(key = key.identifier)
    }

    fun isEmpty() = storage.isEmpty()

    fun contains(value: String): Boolean {
        return storage.any { entry -> entry.value.decodeToString().contains(value) }
    }
}