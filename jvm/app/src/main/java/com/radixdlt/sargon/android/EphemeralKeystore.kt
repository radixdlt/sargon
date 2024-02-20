package com.radixdlt.sargon.android

import com.radixdlt.sargon.SecureStorage
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.secureStorageKeyIdentifier

class EphemeralKeystore: SecureStorage {
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
        return storage.any { entry ->
            entry.value.decodeToString().contains(value)
        }
    }

    private val SecureStorageKey.identifier: String
        get() = secureStorageKeyIdentifier(this)

    override fun toString(): String {
        return storage.toList().joinToString(prefix = "[", postfix = "\n]") { pair ->
            "\n\t${pair.first} => ${pair.second.decodeToString()}"
        }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as EphemeralKeystore

        return storage == other.storage
    }

    override fun hashCode(): Int {
        return storage.hashCode()
    }


}