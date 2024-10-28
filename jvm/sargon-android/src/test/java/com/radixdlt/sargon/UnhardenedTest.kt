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

class UnhardenedTest : SampleTestable<Unhardened> {
    override val samples: List<Sample<Unhardened>>
        get() = listOf(Unhardened.sample)

    @Test
    fun testGlobalOffset() {
        assertEquals(
            Unhardened.globalOffset,
            0u
        )
    }

    @Test
    fun testInit() {
        assertEquals(
            Unhardened.init(U31.init(0u)).indexInGlobalKeySpace,
            Unhardened.globalOffset
        )
    }

    @Test
    fun testInitFromLocal() {
        assertEquals(
            Unhardened.initFromLocal(0u).indexInLocalKeySpace,
            0u
        )
    }

    @Test
    fun testInitFromGlobal() {
        assertEquals(
            Unhardened.initFromGlobal(Unhardened.globalOffset).indexInGlobalKeySpace,
            Unhardened.globalOffset
        )
    }

}