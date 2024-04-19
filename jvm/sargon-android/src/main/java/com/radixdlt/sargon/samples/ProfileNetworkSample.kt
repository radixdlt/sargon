package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Accounts
import com.radixdlt.sargon.AuthorizedDapps
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Personas
import com.radixdlt.sargon.ProfileNetwork
import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
val ProfileNetwork.Companion.sampleMainnet: Sample<ProfileNetwork>
    get() = object : Sample<ProfileNetwork> {
        override fun invoke(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.MAINNET,
            accounts = Accounts.sampleMainnet(),
            personas = Personas.sampleMainnet(),
            authorizedDapps = AuthorizedDapps.sampleMainnet()
        )

        override fun other(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.MAINNET,
            accounts = Accounts.sampleMainnet.other(),
            personas = Personas.sampleMainnet.other(),
            authorizedDapps = AuthorizedDapps.sampleMainnet.other()
        )
    }

@UsesSampleValues
val ProfileNetwork.Companion.sampleStokenet: Sample<ProfileNetwork>
    get() = object : Sample<ProfileNetwork> {
        override fun invoke(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.STOKENET,
            accounts = Accounts.sampleStokenet(),
            personas = Personas.sampleStokenet(),
            authorizedDapps = AuthorizedDapps.sampleStokenet()
        )

        override fun other(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.STOKENET,
            accounts = Accounts.sampleStokenet.other(),
            personas = Personas.sampleStokenet.other(),
            authorizedDapps = AuthorizedDapps.sampleStokenet.other()
        )
    }
