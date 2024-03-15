package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.newPrivateHdFactorSource

fun PrivateHierarchicalDeterministicFactorSource.Companion.init(
    entropy: ByteArray,
    walletClientModel: WalletClientModel
) = newPrivateHdFactorSource(entropy = entropy, walletClientModel = walletClientModel)