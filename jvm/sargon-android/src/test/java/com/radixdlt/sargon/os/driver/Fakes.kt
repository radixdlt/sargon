package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FileSystemDriver
import com.radixdlt.sargon.HostInfoDriver
import com.radixdlt.sargon.HostOs
import com.radixdlt.sargon.LogLevel
import com.radixdlt.sargon.LoggingDriver
import com.radixdlt.sargon.SecureStorageDriver
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.UnsafeStorageDriver
import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.extensions.identifier
import com.radixdlt.sargon.extensions.other

class FakeSecureStorageDriver: SecureStorageDriver {
    private val storage: MutableMap<String, BagOfBytes> = mutableMapOf()

    override suspend fun loadData(key: SecureStorageKey): BagOfBytes? {
        return storage[key.identifier]
    }

    override suspend fun saveData(key: SecureStorageKey, data: BagOfBytes) {
        storage[key.identifier] = data
    }

    override suspend fun deleteDataForKey(key: SecureStorageKey) {
        storage.remove(key.identifier)
    }
}

class FakeUnsafeStorageDriver: UnsafeStorageDriver {
    private val storage: MutableMap<String, BagOfBytes> = mutableMapOf()

    override suspend fun loadData(key: UnsafeStorageKey): BagOfBytes? {
        return storage[key.identifier]
    }

    override suspend fun saveData(key: UnsafeStorageKey, data: BagOfBytes) {
        storage[key.identifier] = data
    }

    override suspend fun deleteDataForKey(key: UnsafeStorageKey) {
        storage.remove(key.identifier)
    }
}

class FakeHostInfoDriver: HostInfoDriver {
    override suspend fun hostOs(): HostOs = HostOs.other(
        name = "host os",
        vendor = "",
        version = "1.0.0"
    )

    override suspend fun hostDeviceName(): String =  "unit"

    override suspend fun hostAppVersion(): String = "1.0.0"

    override suspend fun hostDeviceModel(): String = "test"

}

class FakeLoggingDriver: LoggingDriver {
    override fun log(level: LogLevel, msg: String) {
        println("${level.name} - $msg")
    }
}

class FakeFileSystemDriver: FileSystemDriver {
    private val storage: MutableMap<String, BagOfBytes> = mutableMapOf()

    override suspend fun loadFromFile(path: String): BagOfBytes? {
        return storage[path]
    }

    override suspend fun saveToFile(path: String, data: BagOfBytes, isAllowedToOverwrite: Boolean) {
        storage[path] = data
    }

    override suspend fun deleteFile(path: String) {
        storage.remove(path)
    }

    override suspend fun writableAppDirPath(): String = "fake"
}