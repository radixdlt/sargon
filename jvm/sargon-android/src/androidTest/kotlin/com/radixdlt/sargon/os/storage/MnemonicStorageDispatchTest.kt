package com.radixdlt.sargon.os.storage

import android.os.Looper
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.*
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File
import java.util.UUID

@RunWith(AndroidJUnit4::class)
class MnemonicStorageDispatchTest {
    @Test fun mainThreadCallersRunAllCryptoOnAWorkerThread() = runBlocking<Unit> {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val job = SupervisorJob()
        val file = File(context.cacheDir, "dispatch-${UUID.randomUUID()}.preferences_pb")
        val data = PreferenceDataStoreFactory.create(scope = CoroutineScope(job + Dispatchers.IO)) { file }
        val operations = mutableSetOf<String>()
        val states = mutableMapOf(MnemonicStorage.LEGACY_ALIAS to MnemonicKeyState.Unprotected)
        fun worker(operation: String) {
            assertNotEquals("Blocking $operation ran on Main", Looper.getMainLooper(), Looper.myLooper())
            operations.add(operation)
        }
        val crypto = object : MnemonicStorageCrypto {
            override fun keyState(alias: String): MnemonicKeyState {
                worker("keyState")
                return states[alias] ?: MnemonicKeyState.Missing
            }
            override fun ensureKeyExists(alias: String, mayCreate: Boolean) {
                worker("ensureKeyExists")
                if (alias !in states) {
                    check(mayCreate)
                    states[alias] = MnemonicKeyState.Protected
                }
            }
            override fun delete(alias: String) { worker("delete"); states.remove(alias) }
            override suspend fun encrypt(plaintext: String, alias: String): String {
                worker("encrypt"); return plaintext
            }
            override suspend fun decrypt(ciphertext: String, alias: String): String {
                worker("decrypt"); return ciphertext
            }
        }
        try {
            data.edit { it[stringPreferencesKey("mnemonicA")] = "disposable seed" }
            val storage = MnemonicStorage(data, crypto)
            withContext(Dispatchers.Main) {
                assertEquals(1, storage.pendingMigrationCount().getOrThrow())
                assertEquals(1, storage.migrate().getOrThrow().migrated)
                storage.write("mnemonicB", "another disposable seed").getOrThrow()
                assertEquals("another disposable seed", storage.read("mnemonicB").getOrThrow())
                storage.remove("mnemonicB").getOrThrow()
            }
            assertEquals(setOf("keyState", "ensureKeyExists", "delete", "encrypt", "decrypt"), operations)
        } finally { job.cancelAndJoin(); file.delete() }
    }
}
