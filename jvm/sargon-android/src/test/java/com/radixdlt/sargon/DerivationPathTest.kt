package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DerivationPathTest {

    @Test
    fun testAccountPath() {
        val path = DerivationPath.Cap26(Cap26Path.Account(AccountPath.sample()))
        assertEquals(
            path,
            DerivationPath.Cap26.init(path.string)
        )
    }

    @Test
    fun testIdentityPath() {
        val path = DerivationPath.Cap26(Cap26Path.Identity(IdentityPath.sample()))
        assertEquals(
            path,
            DerivationPath.Cap26.init(path.string)
        )
    }

    @Test
    fun testBip44LikePath() {
        val path = DerivationPath.Bip44Like(Bip44LikePath.sample())
        assertEquals(
            path,
            DerivationPath.Bip44Like.init(path.string)
        )
    }

}