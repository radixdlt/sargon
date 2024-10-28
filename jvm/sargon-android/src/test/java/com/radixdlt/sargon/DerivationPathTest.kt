package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.curve
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.initForEntity
import com.radixdlt.sargon.extensions.initFromLocal
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class DerivationPathTest: SampleTestable<DerivationPath> {
    override val samples: List<Sample<DerivationPath>>
        get() = listOf(DerivationPath.sample)

    @Test
    fun testInit() {
        assertEquals(
            DerivationPath.sample(),
            DerivationPath.initForEntity(
                kind = EntityKind.ACCOUNT,
                networkId = NetworkId.MAINNET,
                index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
            )
        )

        assertEquals(
            DerivationPath.sample.other(),
            DerivationPath.initForEntity(
                kind = EntityKind.PERSONA,
                networkId = NetworkId.MAINNET,
                index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
            )
        )
    }

    @Test
    fun testInitFromPath() {
        assertEquals(
            DerivationPath.init("m/44H/1022H/1H/525H/1460H/0H"),
            AccountPath.init(
                networkId = NetworkId.MAINNET,
                keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
                index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
            ).asGeneral()
        )

        assertEquals(
            DerivationPath.init("m/44H/1022H/1H/525H/1460H/0H"),
            AccountPath.init("m/44H/1022H/1H/525H/1460H/0H").asGeneral()
        )

        assertThrows<CommonException> {
            AccountPath.init("m/44H/1022H/1H/618H/1460H/0H")
        }

        assertThrows<CommonException> {
            AccountPath.init(Bip44LikePath.sample().string)
        }

        assertEquals(
            DerivationPath.init("m/44H/1022H/1H/618H/1460H/0H"),
            IdentityPath.init(
                networkId = NetworkId.MAINNET,
                keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
                index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
            ).asGeneral()
        )

        assertEquals(
            DerivationPath.init("m/44H/1022H/1H/618H/1460H/0H"),
            IdentityPath.init("m/44H/1022H/1H/618H/1460H/0H").asGeneral()
        )

        assertThrows<CommonException> {
            IdentityPath.init("m/44H/1022H/1H/525H/1460H/0H")
        }

        assertThrows<CommonException> {
            IdentityPath.init(Bip44LikePath.sample().string)
        }
    }

    @Test
    fun testCurve() {
        assertEquals(
            Slip10Curve.CURVE25519,
            AccountPath.sample().asGeneral().curve
        )

        assertEquals(
            Slip10Curve.CURVE25519,
            IdentityPath.sample().asGeneral().curve
        )

        assertEquals(
            Slip10Curve.SECP256K1,
            Bip44LikePath.sample().asGeneral().curve
        )
    }

    @Test
    fun testString() {
        assertEquals(
            Bip44LikePath.sample().string,
            Bip44LikePath.sample().asGeneral().string
        )
    }
}