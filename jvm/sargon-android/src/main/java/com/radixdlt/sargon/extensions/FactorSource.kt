package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ArculusCardFactorSource
import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.LedgerHardwareWalletFactorSource
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.OffDeviceMnemonicFactorSource
import com.radixdlt.sargon.PasswordFactorSource
import com.radixdlt.sargon.SpotCheckInput
import com.radixdlt.sargon.factorSourceSupportsBabylon
import com.radixdlt.sargon.factorSourceSupportsOlympia
import com.radixdlt.sargon.factorSourceName
import com.radixdlt.sargon.factorSourcePerformSpotCheck
import com.radixdlt.sargon.newDeviceFactorSourceBabylon
import com.radixdlt.sargon.newDeviceFactorSourceOlympia

val FactorSource.id: FactorSourceId
    get() = when (this) {
        is FactorSource.Device -> value.id.asGeneral()
        is FactorSource.Ledger -> value.id.asGeneral()
        is FactorSource.ArculusCard -> value.id.asGeneral()
        is FactorSource.OffDeviceMnemonic -> value.id.asGeneral()
//        is FactorSource.SecurityQuestions -> value.id.asGeneral()
//        is FactorSource.TrustedContact -> value.id.asGeneral()
        is FactorSource.Password -> value.id.asGeneral()
    }

val FactorSource.kind: FactorSourceKind
    get() = when (this) {
        is FactorSource.Device -> value.kind
        is FactorSource.Ledger -> value.kind
        is FactorSource.ArculusCard -> value.kind
        is FactorSource.OffDeviceMnemonic -> value.kind
//        is FactorSource.SecurityQuestions -> value.kind
//        is FactorSource.TrustedContact -> value.kind
        is FactorSource.Password -> value.kind
    }

fun DeviceFactorSource.asGeneral() = FactorSource.Device(value = this)
fun LedgerHardwareWalletFactorSource.asGeneral() = FactorSource.Ledger(value = this)
fun ArculusCardFactorSource.asGeneral() = FactorSource.ArculusCard(value = this)
fun OffDeviceMnemonicFactorSource.asGeneral() = FactorSource.OffDeviceMnemonic(value = this)
//fun SecurityQuestionsNotProductionReadyFactorSource.asGeneral() =
//    FactorSource.SecurityQuestions(value = this)
//fun TrustedContactFactorSource.asGeneral() = FactorSource.TrustedContact(value = this)
fun PasswordFactorSource.asGeneral() = FactorSource.Password(value = this)

fun FactorSource.Device.Companion.olympia(
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    hostInfo: HostInfo
) = newDeviceFactorSourceOlympia(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    hostInfo = hostInfo
).asGeneral()

fun FactorSource.Device.Companion.babylon(
    mnemonicWithPassphrase: MnemonicWithPassphrase,
    hostInfo: HostInfo
) = newDeviceFactorSourceBabylon(
    mnemonicWithPassphrase = mnemonicWithPassphrase,
    hostInfo = hostInfo
).asGeneral()

fun FactorSource.spotCheck(
    input: SpotCheckInput
) = factorSourcePerformSpotCheck(this, input)

val FactorSource.supportsOlympia: Boolean
    get() = factorSourceSupportsOlympia(factorSource = this)

val FactorSource.supportsBabylon: Boolean
    get() = factorSourceSupportsBabylon(factorSource = this)

val FactorSource.name: String
    get() = factorSourceName(factorSource = this)

val DeviceFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.DEVICE

val LedgerHardwareWalletFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET

val ArculusCardFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.ARCULUS_CARD

val OffDeviceMnemonicFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.OFF_DEVICE_MNEMONIC

//val SecurityQuestionsNotProductionReadyFactorSource.kind: FactorSourceKind
//    get() = FactorSourceKind.SECURITY_QUESTIONS

//val TrustedContactFactorSource.kind: FactorSourceKind
//    get() = FactorSourceKind.TRUSTED_CONTACT

val PasswordFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.PASSWORD

