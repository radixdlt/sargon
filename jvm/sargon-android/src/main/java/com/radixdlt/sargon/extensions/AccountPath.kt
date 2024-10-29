package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountPath
import com.radixdlt.sargon.Cap26EntityKind
import com.radixdlt.sargon.Cap26KeyKind
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.Hardened
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newAccountPath

fun AccountPath.Companion.init(
    networkId: NetworkId,
    keyKind: Cap26KeyKind,
    index: Hardened
): AccountPath = newAccountPath(
    networkId = networkId,
    keyKind = keyKind,
    index = index
)

@Throws(SargonException::class)
fun AccountPath.Companion.init(path: String): AccountPath =
    when (val derivationPath = DerivationPath.init(path)) {
        is DerivationPath.Account -> derivationPath.value
        is DerivationPath.Bip44Like, is DerivationPath.Identity -> throw CommonException.WrongEntityKind(
            expected = Cap26EntityKind.ACCOUNT,
            found = Cap26EntityKind.IDENTITY
        )
    }

fun AccountPath.asGeneral(): DerivationPath = DerivationPath.Account(value = this)