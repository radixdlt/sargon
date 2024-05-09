package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.LedgerHardwareWalletFactorSource
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.deviceFactorSourceIsMainBdfs
import com.radixdlt.sargon.factorSourceSupportsBabylon
import com.radixdlt.sargon.factorSourceSupportsOlympia
import com.radixdlt.sargon.newDeviceFactorSourceBabylon
import com.radixdlt.sargon.newDeviceFactorSourceOlympia

val FactorSource.id: FactorSourceId
    get() = when (this) {
        is FactorSource.Device -> value.id.asGeneral()
        is FactorSource.Ledger -> value.id.asGeneral()
    }

val FactorSource.kind: FactorSourceKind
    get() = when (this) {
        is FactorSource.Device -> value.kind
        is FactorSource.Ledger -> value.kind
    }

fun DeviceFactorSource.asGeneral() = FactorSource.Device(value = this)
fun LedgerHardwareWalletFactorSource.asGeneral() = FactorSource.Ledger(value = this)

fun FactorSource.Device.Companion.olympia(
    mnemonicWithPassphrase: MnemonicWithPassphrase
) = newDeviceFactorSourceOlympia(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    walletClientModel = WalletClientModel.ANDROID
).asGeneral()

fun FactorSource.Device.Companion.babylon(
    isMain: Boolean,
    mnemonicWithPassphrase: MnemonicWithPassphrase
) = newDeviceFactorSourceBabylon(
    isMain = isMain,
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    walletClientModel = WalletClientModel.ANDROID
).asGeneral()

val FactorSource.Device.isMain: Boolean
    get() = deviceFactorSourceIsMainBdfs(deviceFactorSource = value)

val FactorSource.supportsOlympia: Boolean
    get() = factorSourceSupportsOlympia(factorSource = this)

val FactorSource.supportsBabylon: Boolean
    get() = factorSourceSupportsBabylon(factorSource = this)

val DeviceFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.DEVICE

val LedgerHardwareWalletFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET

