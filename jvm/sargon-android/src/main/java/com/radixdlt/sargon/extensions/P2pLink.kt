package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.p2pLinkId

val P2pLink.id: Hash
    get() = p2pLinkId(link = this)