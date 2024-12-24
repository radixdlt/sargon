package com.radixdlt.sargon.os.storage.key

import com.radixdlt.sargon.BagOfBytes

internal interface DatastoreKeyMapping {

    suspend fun write(bagOfBytes: BagOfBytes): Result<Unit>

    suspend fun read(): Result<BagOfBytes?>

    suspend fun remove(): Result<Unit>

    suspend fun keyExist(): Boolean
}