package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class DisplayNameTest {

    @Test
    fun testTrimsWhenLong() {
        assertEquals(
            "jkhfgasdkjhfgskdfghskdghfskdjh",
            DisplayName.init("jkhfgasdkjhfgskdfghskdghfskdjhfgsdkjfhgasdkjhfgsdjkfghaskfhjsd").value,
        )
    }

    @Test
    fun testThrowsWhenEmpty() {
        assertThrows<CommonException> {
            DisplayName.init("")
        }
    }

}