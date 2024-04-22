package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.account
import com.radixdlt.sargon.extensions.addressIndex
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.default
import com.radixdlt.sargon.extensions.hdPath
import com.radixdlt.sargon.extensions.identity
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.nonHardenedIndex
import com.radixdlt.sargon.extensions.nonHardenedValue
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
    fun testBip44LikePathFromIndex() {
        val sut = DerivationPath.Bip44Like.init(index = 0u)
        assertEquals(
            0u,
            sut.addressIndex
        )
    }

    @Test
    fun testNewAccountPathFromElements() {
        assertEquals(
            "m/44H/1022H/1H/525H/1460H/0H",
            DerivationPath.Cap26.account(
                networkId = NetworkId.MAINNET,
                keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
                index = 0u
            ).string
        )
    }

    @Test
    fun testNewIdentityPathFromElements() {
        assertEquals(
            "m/44H/1022H/1H/618H/1460H/0H",
            DerivationPath.Cap26.identity(
                networkId = NetworkId.MAINNET,
                keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
                index = 0u
            ).string
        )
    }

    @Test
    fun testDerivationPathString() {
        val derivationPathCap26: DerivationPath =
            Cap26Path.Account(AccountPath.sample()).asGeneral()
        assertEquals(
            derivationPathCap26.string,
            DerivationPath.Cap26(Cap26Path.Account(AccountPath.sample())).string
        )

        val derivationPathBip44: DerivationPath = Bip44LikePath.sample().asGeneral()
        assertEquals(
            derivationPathBip44.string,
            DerivationPath.Bip44Like(Bip44LikePath.sample()).string
        )
    }

    @Test
    fun testDerivationPathHdPath() {
        val derivationPath = DerivationPath.sample()
        val index = derivationPath.nonHardenedIndex
        assertEquals(
            derivationPath.hdPath.components.last().nonHardenedValue,
            index
        )
    }

}