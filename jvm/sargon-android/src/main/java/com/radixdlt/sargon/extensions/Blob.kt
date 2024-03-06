package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Blob
import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.newBlobFromBytes
import com.radixdlt.sargon.newBlobsFromBlobList

fun Blob.Companion.init(bytes: BagOfBytes) = newBlobFromBytes(bytes = bytes)

fun Blobs.Companion.init(blobs: List<Blob>) = newBlobsFromBlobList(blobs = blobs)

fun Blobs.Companion.init(vararg blob: Blob) = init(blobs = blob.asList())
