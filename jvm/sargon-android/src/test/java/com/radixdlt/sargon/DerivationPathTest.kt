package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.default
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
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

    @Test
    fun testDefaultGetIdPath() {
        assertNotNull(Cap26Path.GetId(GetIdPath.default()).value)
    }

    @Test
    fun testDerivationPathString() {
        val derivationPathCap26: DerivationPath =
            DerivationPath.Cap26(Cap26Path.Account(AccountPath.sample()))
        assertEquals(
            derivationPathCap26.string,
            DerivationPath.Cap26(Cap26Path.Account(AccountPath.sample())).string
        )

        val derivationPathBip44: DerivationPath =
            DerivationPath.Bip44Like(Bip44LikePath.sample())
        assertEquals(
            derivationPathBip44.string,
            DerivationPath.Bip44Like(Bip44LikePath.sample()).string
        )
    }

}