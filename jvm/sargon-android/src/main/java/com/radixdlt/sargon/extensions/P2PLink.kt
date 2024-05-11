package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.p2pLinkId

val P2pLink.id: PublicKeyHash
    get() = p2pLinkId(link = this)