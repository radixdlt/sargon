package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountPath
import com.radixdlt.sargon.Cap26KeyKind
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.EntityKind
import com.radixdlt.sargon.Hardened
import com.radixdlt.sargon.IdentityPath
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.derivationPathToString
import com.radixdlt.sargon.newDerivationPathFromString
import kotlin.jvm.Throws

fun DerivationPath.Companion.initForEntity(
    kind: EntityKind,
    networkId: NetworkId,
    index: Hardened
): DerivationPath = when (kind) {
    EntityKind.ACCOUNT -> AccountPath.init(
        networkId = networkId,
        keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
        index = index
    ).asGeneral()

    EntityKind.PERSONA -> IdentityPath.init(
        networkId = networkId,
        keyKind = Cap26KeyKind.TRANSACTION_SIGNING,
        index = index
    ).asGeneral()
}

@Throws(SargonException::class)
fun DerivationPath.Companion.init(path: String) = newDerivationPathFromString(string = path)

val DerivationPath.string
    get() = derivationPathToString(path = this)

val DerivationPath.curve: Slip10Curve
    get() = when (this) {
        is DerivationPath.Bip44Like -> Slip10Curve.SECP256K1
        is DerivationPath.Account, is DerivationPath.Identity -> Slip10Curve.CURVE25519
    }