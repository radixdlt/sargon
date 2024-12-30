package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Cap26EntityKind
import com.radixdlt.sargon.Cap26KeyKind
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.Hardened
import com.radixdlt.sargon.IdentityPath
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.cap26EntityKindToString
import com.radixdlt.sargon.newIdentityPath

fun IdentityPath.Companion.init(
    networkId: NetworkId,
    keyKind: Cap26KeyKind,
    index: Hardened
): IdentityPath = newIdentityPath(
    networkId = networkId,
    keyKind = keyKind,
    index = index
)

public fun Cap26EntityKind.discriminant(): String = cap26EntityKindToString(kind = this)

@Throws(SargonException::class)
fun IdentityPath.Companion.init(path: String): IdentityPath =
    when (val derivationPath = DerivationPath.init(path)) {
        is DerivationPath.Identity -> derivationPath.value
        is DerivationPath.Bip44Like, is DerivationPath.Account -> throw CommonException.WrongEntityKind(
            expected = Cap26EntityKind.IDENTITY.discriminant(),
            found = Cap26EntityKind.ACCOUNT.discriminant()
        )
    }

fun IdentityPath.asGeneral(): DerivationPath = DerivationPath.Identity(value = this)