package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SavedP2pTransportProfiles
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSavedP2pTransportProfilesSample
import com.radixdlt.sargon.newSavedP2pTransportProfilesSampleOther

@UsesSampleValues
val SavedP2pTransportProfiles.Companion.sample: Sample<SavedP2pTransportProfiles>
    get() = object : Sample<SavedP2pTransportProfiles> {

        override fun invoke(): SavedP2pTransportProfiles = newSavedP2pTransportProfilesSample()

        override fun other(): SavedP2pTransportProfiles = newSavedP2pTransportProfilesSampleOther()
    }
