package com.radixdlt.sargon.os.storage.key

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.newVecOfFactorSourceIdFromHashFromJson
import com.radixdlt.sargon.os.storage.keyExist
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.remove
import com.radixdlt.sargon.os.storage.write
import com.radixdlt.sargon.vecOfFactorSourceIdFromHashToJson

/**
 * Special key mapping class which ports the android logic into sargon.
 *
 * Backed up factor source ids used to be stored on Android as a string array of each ID hex
 * joined together with commas (,).
 * On the other hand sargon expects to serialize a Json array of FactorSourceIdFromHash.
 *
 * This class bridges the difference in handling between Android and iOS maintaining compatibility
 * with existing users.
 */
class FactorSourceUserHasWrittenDownKeyMapping(
    private val storage: DataStore<Preferences>
) : DatastoreKeyMapping {

    private val datastoreKey = stringPreferencesKey(PREFERENCES_KEY)

    override suspend fun write(bagOfBytes: BagOfBytes): Result<Unit> = runCatching {
        // Deserialize BagOfBytes into List<FactorSourceIdFromHash>
        newVecOfFactorSourceIdFromHashFromJson(bagOfBytes)
    }.map { ids ->
        // Write as a comma separated string of ids hex.
        writeAndroidBackedUpFactorSourceIds(ids)
    }

    override suspend fun read(): Result<BagOfBytes?> =
        // Read the comma separated list
        readAndroidBackedUpFactorSourceIds()
            .mapCatching { idsOrNull ->
                idsOrNull?.let { ids ->
                    // Convert into BagOfBytes so the result can be passed on to sargon.
                    vecOfFactorSourceIdFromHashToJson(ids)
                }
            }

    override suspend fun remove(): Result<Unit> = storage.remove(datastoreKey)

    override suspend fun keyExist(): Boolean = storage.keyExist(datastoreKey)

    /**
     * Reads the comma separated string of factor source ids hex
     */
    private suspend fun readAndroidBackedUpFactorSourceIds(): Result<List<FactorSourceIdFromHash>?> =
        storage.read(datastoreKey)
            .mapCatching { hexCommaSeparated ->
                hexCommaSeparated
                    ?.split(ANDROID_SEPARATOR)
                    ?.map { id ->
                        FactorSourceIdFromHash(
                            kind = FactorSourceKind.DEVICE,
                            body = Exactly32Bytes.init(id.hexToBagOfBytes())
                        )
                    }
            }

    /**
     * Writes into the android storage as a comma separated list of hex ids. This maintains
     * compatibility with the current implementation.
     */
    private suspend fun writeAndroidBackedUpFactorSourceIds(
        ids: List<FactorSourceIdFromHash>
    ): Result<Unit> = storage.write(
        datastoreKey,
        ids.joinToString(separator = ANDROID_SEPARATOR) { it.body.hex }
    )

    companion object {
        private const val PREFERENCES_KEY = "backed_up_factor_source_ids"
        private const val ANDROID_SEPARATOR = ","
    }

}