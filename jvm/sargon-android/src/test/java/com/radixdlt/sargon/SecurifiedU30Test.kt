package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.globalOffset
import com.radixdlt.sargon.extensions.indexInGlobalKeySpace
import com.radixdlt.sargon.extensions.indexInLocalKeySpace
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.initFromGlobal
import com.radixdlt.sargon.extensions.initFromLocal
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SecurifiedU30Test : SampleTestable<SecurifiedU30> {
    override val samples: List<Sample<SecurifiedU30>>
        get() = listOf(SecurifiedU30.sample)

    @Test
    fun testGlobalOffset() {
        assertEquals(
            SecurifiedU30.globalOffset,
            3221225472u
        )
    }

    @Test
    fun testInit() {
        assertEquals(
            SecurifiedU30.init(U30.init(0u)).indexInGlobalKeySpace,
            SecurifiedU30.globalOffset
        )
    }

    @Test
    fun testInitFromLocal() {
        assertEquals(
            SecurifiedU30.initFromLocal(0u).indexInLocalKeySpace,
            0u
        )
    }

    @Test
    fun testInitFromGlobal() {
        assertEquals(
            SecurifiedU30.initFromGlobal(SecurifiedU30.globalOffset).indexInGlobalKeySpace,
            SecurifiedU30.globalOffset
        )
    }

}