package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.LedgerHardwareWalletModel
import com.radixdlt.sargon.ledgerHwWalletModelToString
import com.radixdlt.sargon.newLedgerHwWalletModelFromString

@Throws(SargonException::class)
fun LedgerHardwareWalletModel.Companion.init(string: String) =
    newLedgerHwWalletModelFromString(string = string)

val LedgerHardwareWalletModel.string: String
    get() = ledgerHwWalletModelToString(model = this)