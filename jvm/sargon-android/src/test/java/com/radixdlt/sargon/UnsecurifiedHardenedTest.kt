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

class UnsecurifiedHardenedTest : SampleTestable<UnsecurifiedHardened> {
    override val samples: List<Sample<UnsecurifiedHardened>>
        get() = listOf(UnsecurifiedHardened.sample)

    @Test
    fun testGlobalOffset() {
        assertEquals(
            UnsecurifiedHardened.globalOffset,
            2147483648u
        )
    }

    @Test
    fun testInit() {
        assertEquals(
            UnsecurifiedHardened.init(U30.init(0u)).indexInGlobalKeySpace,
            UnsecurifiedHardened.globalOffset
        )
    }

    @Test
    fun testInitFromLocal() {
        assertEquals(
            UnsecurifiedHardened.initFromLocal(0u).indexInLocalKeySpace,
            0u
        )
    }

    @Test
    fun testInitFromGlobal() {
        assertEquals(
            UnsecurifiedHardened.initFromGlobal(UnsecurifiedHardened.globalOffset).indexInGlobalKeySpace,
            UnsecurifiedHardened.globalOffset
        )
    }

}