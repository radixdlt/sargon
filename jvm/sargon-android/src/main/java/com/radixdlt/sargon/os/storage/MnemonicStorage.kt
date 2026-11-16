package com.radixdlt.sargon.os.storage

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.os.driver.AndroidBiometricAuthorizationDriver
import com.radixdlt.sargon.os.driver.BiometricsFailure
import com.radixdlt.sargon.os.driver.BiometricsHandler
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext

/** One instance must be shared by the wallet and its Android storage driver. */
class MnemonicStorage internal constructor(
    private val preferences: DataStore<Preferences>,
    private val crypto: MnemonicStorageCrypto
) {
    constructor(preferences: DataStore<Preferences>, biometricsHandler: BiometricsHandler) : this(
        preferences, AndroidMnemonicStorageCrypto(AndroidBiometricAuthorizationDriver(biometricsHandler))
    )

    private val mutex = Mutex()

    data class MigrationResult(
        val migrated: Int,
        val remaining: Int,
        val cleanupPending: Boolean
    )

    suspend fun read(name: String): Result<String?> = operation {
        storedValue(name)?.let { stored ->
            crypto.decrypt(stored.removePrefix(PREFIX), alias(stored))
        }
    }

    suspend fun write(name: String, plaintext: String): Result<Unit> = operation {
        if (crypto.keyState(PROTECTED_ALIAS) == MnemonicKeyState.Missing) {
            ensureProtectedKeyExists(records())
        }
        val encrypted = crypto.encrypt(plaintext, PROTECTED_ALIAS)
        preferences.edit {
            it[stringPreferencesKey(name)] = envelope(encrypted)
        }
        Unit
    }

    suspend fun remove(name: String): Result<Unit> = operation {
        preferences.edit { it.remove(stringPreferencesKey(name)) }
        Unit
    }

    suspend fun contains(name: String): Boolean = storedValue(name) != null

    /** Restores the original invalidation/reset policy, removing records for each reset alias. */
    suspend fun clearInvalidatedMnemonics(): Result<Unit> = operation {
        val byAlias = records().entries.groupBy { (_, value) ->
            (value as? String)?.let { runCatching { alias(it) }.getOrNull() }
        }
        for ((alias, records) in byAlias) {
            if (alias == null || !crypto.isInvalidated(alias)) continue
            // Reset the key before clearing the records that depended on it.
            crypto.reset(alias)
            preferences.edit { prefs ->
                for ((name, stored) in records) {
                    val key = stringPreferencesKey(name)
                    if (prefs[key] == stored) prefs.remove(key)
                }
            }
        }
    }

    suspend fun pendingMigrationCount(): Result<Int> = operation {
        if (crypto.keyState(LEGACY_ALIAS) == MnemonicKeyState.Unprotected) records().values.count { !isV2(it) }
        else 0
    }

    /** Commits one re-encrypted record at a time. Cancellation never rolls back earlier commits. */
    suspend fun migrate(): Result<MigrationResult> = operation {
        if (crypto.keyState(LEGACY_ALIAS) != MnemonicKeyState.Unprotected) {
            return@operation MigrationResult(0, 0, false)
        }
        var migrated = 0
        val original = records()
        val pending = original.filterValues { !isV2(it) }
        if (pending.isNotEmpty()) ensureProtectedKeyExists(original)
        for ((name, stored) in pending.toSortedMap()) {
            try {
                val plaintext = crypto.decrypt(stored as? String ?: error("Unsupported mnemonic record"), LEGACY_ALIAS)
                val encrypted = crypto.encrypt(plaintext, PROTECTED_ALIAS)
                preferences.edit {
                    val key = stringPreferencesKey(name)
                    check(it[key] == stored) { "Mnemonic changed during migration" }
                    it[key] = envelope(encrypted)
                }
                migrated++
            } catch (cancelled: CancellationException) {
                throw cancelled
            } catch (_: BiometricsFailure) {
                break
            } catch (_: Exception) {
                // Keep the original record and continue securing the other mnemonics.
            }
        }
        val remaining = pending.size - migrated
        var cleanupPending = false
        if (remaining == 0) {
            try {
                // Key deletion has no regeneration fallback. Retrying after a committed migration is safe.
                crypto.delete(LEGACY_ALIAS)
            } catch (cancelled: CancellationException) {
                throw cancelled
            } catch (_: Exception) {
                cleanupPending = true
            }
        }
        MigrationResult(migrated, remaining, cleanupPending)
    }

    private suspend fun storedValue(name: String): String? =
        preferences.data.first()[stringPreferencesKey(name)]

    private suspend fun records(): Map<String, Any> = preferences.data.first().asMap()
        .filterKeys { it.name.startsWith("mnemonic") }
        .mapKeys { (key, _) -> key.name }

    private fun isV2(value: Any) = value is String && value.startsWith(PREFIX)
    private fun alias(value: String) = if (isV2(value)) PROTECTED_ALIAS else {
        check(!value.startsWith("sargon:")) { "Unsupported mnemonic storage version" }
        LEGACY_ALIAS
    }

    private fun envelope(ciphertext: String) = "$PREFIX$ciphertext"

    private fun ensureProtectedKeyExists(records: Map<String, Any>) {
        crypto.ensureKeyExists(PROTECTED_ALIAS, mayCreate = records.values.none(::isV2))
    }

    private suspend fun <T> operation(block: suspend () -> T): Result<T> = withContext(Dispatchers.IO) {
        // Serialize access with migration and keep blocking Keystore operations off Main.
        mutex.withLock {
            try {
                Result.success(block())
            } catch (cancelled: CancellationException) {
                throw cancelled
            } catch (error: Exception) {
                Result.failure(error)
            }
        }
    }

    internal companion object {
        const val LEGACY_ALIAS = "EncryptedMnemonicAlias"
        const val PROTECTED_ALIAS = "EncryptedMnemonicAliasV2"
        const val PREFIX = "sargon:mnemonic:v2:"
    }
}

internal enum class MnemonicKeyState { Missing, Unprotected, Protected }

internal interface MnemonicStorageCrypto {
    fun keyState(alias: String): MnemonicKeyState
    fun isInvalidated(alias: String): Boolean = keyState(alias) == MnemonicKeyState.Missing
    fun reset(alias: String) = delete(alias)
    /** Reuses an existing key, or creates an authentication-required key when [mayCreate] is true. */
    fun ensureKeyExists(alias: String, mayCreate: Boolean)
    fun delete(alias: String)
    suspend fun encrypt(plaintext: String, alias: String): String
    /** Requests user authorization even for legacy keys that do not require it cryptographically. */
    suspend fun decrypt(ciphertext: String, alias: String): String
}
