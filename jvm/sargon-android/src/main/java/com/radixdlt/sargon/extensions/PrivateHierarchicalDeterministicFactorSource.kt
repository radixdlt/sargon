package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceInfo
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
    deviceInfo: DeviceInfo
) = newPrivateHdFactorSourceBabylon(
    isMain = isMainBDFS,
    entropy = entropy,
    deviceInfo = deviceInfo
)


fun PrivateHierarchicalDeterministicFactorSource.Companion.olympia(
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    deviceInfo: DeviceInfo
) = newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    deviceInfo = deviceInfo
)

fun PrivateHierarchicalDeterministicFactorSource.Companion.babylon(
    isMain: Boolean,
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    deviceInfo: DeviceInfo
) = newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
    isMain = isMain,
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    deviceInfo = deviceInfo
)

