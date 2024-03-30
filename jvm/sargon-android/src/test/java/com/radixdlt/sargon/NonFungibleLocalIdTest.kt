package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.bytesId
import com.radixdlt.sargon.extensions.intId
import com.radixdlt.sargon.extensions.ruidId
import com.radixdlt.sargon.extensions.stringId
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleLocalIdTest {

    @Test
    fun test() {
        val hex = "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210"
        val bagOfBytes = hex.hexToBagOfBytes()
        assertEquals("#1234#", NonFungibleLocalId.intId(value = 1234.toULong()).string)
        assertEquals("<foo>", NonFungibleLocalId.stringId(string = "foo").string)
        assertEquals(
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}",
            NonFungibleLocalId.ruidId(value = bagOfBytes).string
        )
        assertEquals(
            "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]",
            NonFungibleLocalId.bytesId(bytes = bagOfBytes).string
        )
    }

    @Test
    fun testLocalIdFromString() {
        assertEquals("#1234#", NonFungibleLocalId.init(localId = "#1234#").string)
        assertEquals("<foo>", NonFungibleLocalId.init(localId = "<foo>").string)
        assertEquals(
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}",
            NonFungibleLocalId.init(localId = "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}").string
        )
        assertEquals(
            "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]",
            NonFungibleLocalId.init(localId = "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]").string
        )
    }

    @Test
    fun testFormat() {
        val address = NonFungibleLocalId.init(localId = "#1234#")

        assertEquals("1234", address.formatted())
        assertEquals(
            "1234",
            address.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            address.string,
            address.formatted(format = AddressFormat.RAW)
        )
    }

}