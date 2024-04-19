package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NonEmptyMax32Bytes
import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.newPrivateHdFactorSource

@Throws(SargonException::class)
fun PrivateHierarchicalDeterministicFactorSource.Companion.init(
    entropy: NonEmptyMax32Bytes,
    walletClientModel: WalletClientModel
) = newPrivateHdFactorSource(entropy = entropy, walletClientModel = walletClientModel)