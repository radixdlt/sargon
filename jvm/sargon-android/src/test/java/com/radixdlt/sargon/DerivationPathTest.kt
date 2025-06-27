package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.bip32String
import com.radixdlt.sargon.extensions.curve
import com.radixdlt.sargon.extensions.displayString
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.initForEntity
import com.radixdlt.sargon.extensions.initFromLocal
import com.radixdlt.sargon.extensions.lastPathComponent
import com.radixdlt.sargon.extensions.scheme
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
    fun testScheme() {
        assertEquals(
            DerivationPathScheme.CAP26,
            AccountPath.sample().asGeneral().scheme
        )

        assertEquals(
            DerivationPathScheme.CAP26,
            IdentityPath.sample().asGeneral().scheme
        )

        assertEquals(
            DerivationPathScheme.BIP44_OLYMPIA,
            Bip44LikePath.sample().asGeneral().scheme
        )
    }

    @Test
    fun testString() {
        val accountPathInSecurifiedSpace = AccountPath.init(
            networkId = NetworkId.MAINNET,
            keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
            index = Hardened.Securified(SecurifiedU30.initFromLocal(0u))
        ).asGeneral()

        assertEquals(
            "m/44H/1022H/1H/525H/1460H/0S",
            accountPathInSecurifiedSpace.displayString
        )
        assertEquals(
            "m/44H/1022H/1H/525H/1460H/1073741824H",
            accountPathInSecurifiedSpace.bip32String
        )

        val accountPathInUnsecurifiedSpace = AccountPath.init(
            networkId = NetworkId.MAINNET,
            keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
            index = Hardened.Unsecurified(UnsecurifiedHardened(U30.init(0u)))
        ).asGeneral()

        assertEquals(
            "m/44H/1022H/1H/525H/1460H/0H",
            accountPathInUnsecurifiedSpace.displayString
        )
        assertEquals(
            "m/44H/1022H/1H/525H/1460H/0H",
            accountPathInUnsecurifiedSpace.bip32String
        )
    }

    @Test
    fun testLastPathComponent() {
        assertEquals(
            AccountPath.init(
                networkId = NetworkId.MAINNET,
                keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
                index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
            ).asGeneral().lastPathComponent,
            HdPathComponent.init(
                localKeySpace = 0u,
                keySpace = KeySpace.Unsecurified(isHardened = true)
            )
        )

        assertEquals(
            IdentityPath.init(
                networkId = NetworkId.MAINNET,
                keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
                index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
            ).asGeneral().lastPathComponent,
            HdPathComponent.init(
                localKeySpace = 0u,
                keySpace = KeySpace.Unsecurified(isHardened = true)
            )
        )

        assertEquals(
            Bip44LikePath.init(
                index = HdPathComponent.init(
                    localKeySpace = 0u,
                    keySpace = KeySpace.Unsecurified(isHardened = true)
                )
            ).asGeneral().lastPathComponent,
            HdPathComponent.init(
                localKeySpace = 0u,
                keySpace = KeySpace.Unsecurified(isHardened = true)
            )
        )
    }
}