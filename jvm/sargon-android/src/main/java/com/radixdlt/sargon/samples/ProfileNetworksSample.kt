package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Accounts
import com.radixdlt.sargon.AuthorizedDapps
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Personas
import com.radixdlt.sargon.ProfileNetwork
import com.radixdlt.sargon.ProfileNetworks
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val ProfileNetworks.Companion.sampleMainnet: Sample<ProfileNetworks>
    get() = object : Sample<ProfileNetworks> {
        override fun invoke(): ProfileNetworks = ProfileNetworks.init(
            ProfileNetwork.sampleMainnet(), ProfileNetwork.sampleMainnet.other()
        )

        override fun other(): ProfileNetworks = ProfileNetworks.init(
            ProfileNetwork.sampleMainnet()
        )
    }

@UsesSampleValues
val ProfileNetworks.Companion.sampleStokenet: Sample<ProfileNetworks>
    get() = object : Sample<ProfileNetworks> {
        override fun invoke(): ProfileNetworks = ProfileNetworks.init(
            ProfileNetwork.sampleStokenet(), ProfileNetwork.sampleStokenet.other()
        )

        override fun other(): ProfileNetworks = ProfileNetworks.init(
            ProfileNetwork.sampleStokenet()
        )
    }
