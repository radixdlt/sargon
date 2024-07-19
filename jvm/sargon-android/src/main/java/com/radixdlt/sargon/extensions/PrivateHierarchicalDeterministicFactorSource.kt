package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.NonEmptyMax32Bytes
import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.newPrivateHdFactorSourceBabylon
import com.radixdlt.sargon.newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase
import com.radixdlt.sargon.newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase

@Throws(SargonException::class)
fun PrivateHierarchicalDeterministicFactorSource.Companion.init(
    isMainBDFS: Boolean,
    entropy: NonEmptyMax32Bytes,
    hostInfo: HostInfo
) = newPrivateHdFactorSourceBabylon(
    isMain = isMainBDFS,
    entropy = entropy,
    hostInfo = hostInfo
)


fun PrivateHierarchicalDeterministicFactorSource.Companion.olympia(
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    hostInfo: HostInfo
) = newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    hostInfo = hostInfo
)

fun PrivateHierarchicalDeterministicFactorSource.Companion.babylon(
    isMain: Boolean,
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    hostInfo: HostInfo
) = newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
    isMain = isMain,
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    hostInfo = hostInfo
)

