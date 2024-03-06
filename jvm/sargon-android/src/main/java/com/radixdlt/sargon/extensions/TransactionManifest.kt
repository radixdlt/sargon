package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.TransactionManifest
import com.radixdlt.sargon.newTransactionManifestFromInstructionsStringAndBlobs
import com.radixdlt.sargon.transactionManifestToString

fun TransactionManifest.Companion.init(
    instructionsString: String,
    networkId: NetworkId,
    blobs: Blobs
) = newTransactionManifestFromInstructionsStringAndBlobs(
    instructionsString = instructionsString,
    networkId = networkId,
    blobs = blobs
)

val TransactionManifest.string: String
    get() = transactionManifestToString(manifest = this)