package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.WalletToDappInteractionResponse
import com.radixdlt.sargon.newWalletToDappInteractionResponseFromJsonBytes
import com.radixdlt.sargon.walletToDappInteractionResponseToJsonBytes

fun WalletToDappInteractionResponse.Companion.fromJson(json: String) = runCatching {
    newWalletToDappInteractionResponseFromJsonBytes(json.toByteArray().toBagOfBytes())
}

fun WalletToDappInteractionResponse.toJson() = runCatching {
    walletToDappInteractionResponseToJsonBytes(this).string
}