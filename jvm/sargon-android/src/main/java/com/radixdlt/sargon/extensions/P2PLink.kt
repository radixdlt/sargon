package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.newP2PLinkFromJsonBytes
import com.radixdlt.sargon.p2PLinkToJsonBytes
import com.radixdlt.sargon.p2pLinkId

val P2pLink.id: PublicKeyHash
    get() = p2pLinkId(link = this)

@Throws(SargonException::class)
fun P2pLink.Companion.fromJson(json: String) =
    newP2PLinkFromJsonBytes(jsonBytes = bagOfBytes(json))

fun P2pLink.toJson(): String =
    p2PLinkToJsonBytes(this).string

fun P2pLink.clientID(): Hash =
    connectionPassword.value.bytes.hash()