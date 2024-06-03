package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.WalletToDappInteractionResponse
import com.radixdlt.sargon.walletToDappInteractionResponseToJsonBytes

fun WalletToDappInteractionResponse.asJsonString() = runCatching {
    walletToDappInteractionResponseToJsonBytes(this).string
}