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
    entropy: NonEmptyMax32Bytes,
    hostInfo: HostInfo
) = newPrivateHdFactorSourceBabylon(
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
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    hostInfo: HostInfo
) = newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    hostInfo = hostInfo
)

