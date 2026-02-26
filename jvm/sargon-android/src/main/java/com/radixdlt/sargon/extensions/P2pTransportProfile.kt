package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.P2pTransportProfile
import com.radixdlt.sargon.p2pTransportProfileId

val P2pTransportProfile.id: String
    get() = p2pTransportProfileId(profile = this)
