package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asHardened
import com.radixdlt.sargon.extensions.bip32String
import com.radixdlt.sargon.extensions.globalOffset
import com.radixdlt.sargon.extensions.indexInGlobalKeySpace
import com.radixdlt.sargon.extensions.indexInLocalKeySpace
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.initFromLocal
import com.radixdlt.sargon.extensions.keySpace
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class HDPathComponentTest: SampleTestable<HdPathComponent> {

    override val samples: List<Sample<HdPathComponent>>
        get() = listOf(HdPathComponent.sample)

    @Test
    fun testInit() {
        assertEquals(
            HdPathComponent.init(globalKeySpace = 0u),
            HdPathComponent.init(
                localKeySpace = 0u,
                keySpace = KeySpace.Unsecurified(isHardened = false))
        )

        assertEquals(
            HdPathComponent.init(globalKeySpace = SecurifiedU30.globalOffset),
            HdPathComponent.init(
                localKeySpace = 0u,
                keySpace = KeySpace.Securified
            )
        )
    }

    @Test
    fun testString() {
        assertEquals(
            HdPathComponent.sample().bip32String,
            "237"
        )
    }

    @Test
    fun testKeySpace() {
        assertEquals(
            HdPathComponent.sample().keySpace,
            KeySpace.Unsecurified(isHardened = false)
        )

        assertEquals(
            HdPathComponent.sample.other().keySpace,
            KeySpace.Securified
        )
    }

    @Test
    fun testIndex() {
        assertEquals(
            HdPathComponent.sample.other().indexInLocalKeySpace,
            1073741823u
        )
        assertEquals(
            HdPathComponent.sample.other().indexInGlobalKeySpace,
            4294967295u
        )
    }

    @Test
    fun testIsHardened() {
        assertEquals(
            HdPathComponent.sample.other().asHardened(),
            Hardened.Securified(SecurifiedU30.init(U30.init(1073741823u)))
        )
    }

}