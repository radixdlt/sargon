package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonEmptyMax32Bytes
import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.newPrivateHdFactorSourceBabylon

@Throws(SargonException::class)
fun PrivateHierarchicalDeterministicFactorSource.Companion.init(
    isMainBDFS: Boolean,
    entropy: NonEmptyMax32Bytes,
    walletClientModel: WalletClientModel
) = newPrivateHdFactorSourceBabylon(isMain = isMainBDFS, entropy = entropy, walletClientModel = walletClientModel)