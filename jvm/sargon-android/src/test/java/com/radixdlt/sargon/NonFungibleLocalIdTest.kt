package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleLocalIdTest {

    @Test
    fun test() {
        val hex = "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210"
        val bagOfBytes = hex.hexToBagOfBytes()
        assertEquals("#1234#", NonFungibleLocalId.Integer(value = 1234.toULong()).string)
        assertEquals("<foo>", newNonFungibleLocalIdString(string = "foo").string)
        assertEquals(
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}",
            newNonFungibleLocalIdRuid(bytes = bagOfBytes).string
        )
        assertEquals(
            "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]",
            newNonFungibleLocalIdBytes(bytes = bagOfBytes).string
        )
    }

}