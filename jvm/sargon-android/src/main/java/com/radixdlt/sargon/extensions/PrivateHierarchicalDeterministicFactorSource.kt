package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.NonEmptyMax32Bytes
import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.newPrivateHdFactorSourceBabylon
import com.radixdlt.sargon.newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase
import com.radixdlt.sargon.newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase

@Throws(SargonException::class)
fun PrivateHierarchicalDeterministicFactorSource.Companion.init(
    isMainBDFS: Boolean,
    entropy: NonEmptyMax32Bytes,
    walletClientModel: WalletClientModel
) = newPrivateHdFactorSourceBabylon(
    isMain = isMainBDFS,
    entropy = entropy,
    walletClientModel = walletClientModel
)


fun PrivateHierarchicalDeterministicFactorSource.Companion.olympia(
    mnemonicWithPassphrase: MnemonicWithPassphrase
) = newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    walletClientModel = WalletClientModel.ANDROID
)

fun PrivateHierarchicalDeterministicFactorSource.Companion.babylon(
    isMain: Boolean,
    mnemonicWithPassphrase: MnemonicWithPassphrase
) = newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
    isMain = isMain,
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    walletClientModel = WalletClientModel.ANDROID
)

