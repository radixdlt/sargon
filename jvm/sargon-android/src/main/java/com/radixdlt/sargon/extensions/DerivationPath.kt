package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountPath
import com.radixdlt.sargon.Cap26KeyKind
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.DerivationPathScheme
import com.radixdlt.sargon.EntityKind
import com.radixdlt.sargon.Hardened
import com.radixdlt.sargon.HdPath
import com.radixdlt.sargon.HdPathComponent
import com.radixdlt.sargon.IdentityPath
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.derivationPathToBip32String
import com.radixdlt.sargon.derivationPathToHdPath
import com.radixdlt.sargon.derivationPathToString

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

val DerivationPath.displayString
    get() = derivationPathToString(path = this)

val DerivationPath.bip32String: String
    get() = derivationPathToBip32String(path = this)

val DerivationPath.path: HdPath
    get() = derivationPathToHdPath(path = this)

val DerivationPath.curve: Slip10Curve
    get() = when (this) {
        is DerivationPath.Bip44Like -> Slip10Curve.SECP256K1
        is DerivationPath.Account, is DerivationPath.Identity -> Slip10Curve.CURVE25519
    }

val DerivationPath.lastPathComponent: HdPathComponent
    get() = path.components.last()

val DerivationPath.scheme: DerivationPathScheme
    get() = when (this) {
        is DerivationPath.Bip44Like -> DerivationPathScheme.BIP44_OLYMPIA
        is DerivationPath.Account,
        is DerivationPath.Identity -> DerivationPathScheme.CAP26
    }