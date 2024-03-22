package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.from
import com.radixdlt.sargon.extensions.fromBytes
import com.radixdlt.sargon.extensions.fromInt
import com.radixdlt.sargon.extensions.fromRuid
import com.radixdlt.sargon.extensions.fromString
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleLocalIdTest {

    @Test
    fun test() {
        val hex = "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210"
        val bagOfBytes = hex.hexToBagOfBytes()
        assertEquals("#1234#", NonFungibleLocalId.fromInt(value = 1234.toULong()).string)
        assertEquals("<foo>", NonFungibleLocalId.fromString(string = "foo").string)
        assertEquals(
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}",
            NonFungibleLocalId.fromRuid(value = bagOfBytes).string
        )
        assertEquals(
            "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]",
            NonFungibleLocalId.fromBytes(bytes = bagOfBytes).string
        )
    }

    @Test
    fun testLocalIdFromString() {
        assertEquals("#1234#", NonFungibleLocalId.from(localId = "#1234#").string)
        assertEquals("<foo>", NonFungibleLocalId.from(localId = "<foo>").string)
        assertEquals(
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}",
            NonFungibleLocalId.from(localId = "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}").string
        )
        assertEquals(
            "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]",
            NonFungibleLocalId.from(localId = "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]").string
        )
    }

}