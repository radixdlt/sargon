package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toList
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class BlobsTest {

    @Test
    fun test() {
        val bytes = randomBagOfBytes(byteCount = 32)
        val blob = Blob.init(bytes)

        assertEquals(bytes, blob.bytes)
        assertEquals(bytes.hex, blob.string)
        assertEquals(blob, Blob.init(bytes))

        val anotherBytes = randomBagOfBytes(byteCount = 32)
        val anotherBlob = Blob.init(anotherBytes)
        val blobs = Blobs.init(blob, anotherBlob)

        assertEquals(listOf(blob, anotherBlob), blobs.toList())
    }

}