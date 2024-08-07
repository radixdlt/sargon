package com.radixdlt.sargon.android

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.SecureStorageDriver
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.identifier

class EphemeralKeystore: SecureStorageDriver {
    private val storage: MutableMap<String, BagOfBytes> = mutableMapOf()

    override suspend fun loadData(key: SecureStorageKey): BagOfBytes? = storage[key.identifier]

    override suspend fun saveData(key: SecureStorageKey, data: BagOfBytes) {
        storage[key.identifier] = data
    }

    override suspend fun deleteDataForKey(key: SecureStorageKey) {
        storage.remove(key = key.identifier)
    }

    fun isEmpty() = storage.isEmpty()

//    fun contains(value: String): Boolean {
//        return storage.any { entry ->
//            entry.value().decodeToString().contains(value)
//        }
//    }
//
//    override fun toString(): String {
//        return storage.toList().joinToString(prefix = "[", postfix = "\n]") { pair ->
//            "\n\t${pair.first} => ${pair.second.decodeToString()}"
//        }
//    }

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