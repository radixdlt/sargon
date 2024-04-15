package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.LedgerHardwareWalletFactorSource

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

val DeviceFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.DEVICE

val LedgerHardwareWalletFactorSource.kind: FactorSourceKind
    get() = FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET

