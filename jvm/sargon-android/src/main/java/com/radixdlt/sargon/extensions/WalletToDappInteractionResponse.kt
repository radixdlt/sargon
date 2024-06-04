package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.WalletToDappInteractionResponse
import com.radixdlt.sargon.newWalletToDappInteractionResponseFromJsonBytes
import com.radixdlt.sargon.walletToDappInteractionResponseToJsonBytes

@Throws(SargonException::class)
fun WalletToDappInteractionResponse.Companion.fromJson(json: String) =
    newWalletToDappInteractionResponseFromJsonBytes(json.toByteArray().toBagOfBytes())

@Throws(SargonException::class)
fun WalletToDappInteractionResponse.toJson() =
    walletToDappInteractionResponseToJsonBytes(this).string