package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.addressIndex
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.displayString
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class BIP44LikePathTest: SampleTestable<Bip44LikePath> {
    override val samples: List<Sample<Bip44LikePath>>
        get() = listOf(Bip44LikePath.sample)

    @Test
    fun testRoundtrip() {
        assertEquals(
            Bip44LikePath.sample(),
            Bip44LikePath.init(Bip44LikePath.sample().addressIndex)
        )

        assertEquals(
            Bip44LikePath.sample(),
            Bip44LikePath.init(Bip44LikePath.sample().index)
        )
    }

    @Test
    fun testAsGeneral() {
        assertEquals(
            DerivationPath.Bip44Like(Bip44LikePath.sample()),
            Bip44LikePath.sample().asGeneral(),
        )
    }

    @Test
    fun testIndex() {
        assertEquals(
            Bip44LikePath.sample().addressIndex,
            HdPathComponent.init(localKeySpace = 0u, KeySpace.Unsecurified(isHardened = false)),
        )
    }

    @Test
    fun testDisplayString() {
        assertEquals(
            "m/44H/1022H/0H/0/0",
            Bip44LikePath.sample().displayString,
        )
    }
}