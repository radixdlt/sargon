package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DappToWalletInteractionUnvalidated
import com.radixdlt.sargon.dappToWalletInteractionUnvalidatedToJsonBytes
import com.radixdlt.sargon.newDappToWalletInteractionUnvalidatedFromJsonBytes

@Throws(SargonException::class)
fun DappToWalletInteractionUnvalidated.Companion.fromJson(json: String) =
    newDappToWalletInteractionUnvalidatedFromJsonBytes(json.toByteArray().toBagOfBytes())

@Throws(SargonException::class)
fun DappToWalletInteractionUnvalidated.toJson() =
    dappToWalletInteractionUnvalidatedToJsonBytes(this).string

