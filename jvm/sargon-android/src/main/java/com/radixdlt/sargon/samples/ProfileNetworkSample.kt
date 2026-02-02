package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.ProfileNetwork
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.Accounts
import com.radixdlt.sargon.extensions.AuthorizedDapps
import com.radixdlt.sargon.extensions.MfaFactorInstances
import com.radixdlt.sargon.extensions.Personas
import com.radixdlt.sargon.extensions.ResourceAppPreferences

@UsesSampleValues
val ProfileNetwork.Companion.sampleMainnet: Sample<ProfileNetwork>
    get() = object : Sample<ProfileNetwork> {
        override fun invoke(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.MAINNET,
            accounts = Accounts(
                Account.sampleMainnet(),
            ).asList(),
            personas = Personas(
                Persona.sampleMainnet()
            ).asList(),
            authorizedDapps = AuthorizedDapps(
                AuthorizedDapp.sampleMainnet()
            ).asList(),
            resourcePreferences = ResourceAppPreferences.sample().asList(),
            mfaFactorInstances = MfaFactorInstances.sample().asList()
        )

        override fun other(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.MAINNET,
            accounts = Accounts(
                Account.sampleMainnet.other(),
            ).asList(),
            personas = Personas(
                Persona.sampleMainnet.other()
            ).asList(),
            authorizedDapps = AuthorizedDapps(
                AuthorizedDapp.sampleMainnet.other()
            ).asList(),
            resourcePreferences = ResourceAppPreferences.sample.other().asList(),
            mfaFactorInstances = MfaFactorInstances.sample().asList()
        )
    }

@UsesSampleValues
val ProfileNetwork.Companion.sampleStokenet: Sample<ProfileNetwork>
    get() = object : Sample<ProfileNetwork> {
        override fun invoke(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.STOKENET,
            accounts = Accounts(
                Account.sampleStokenet(),
            ).asList(),
            personas = Personas(
                Persona.sampleStokenet()
            ).asList(),
            authorizedDapps = AuthorizedDapps(
                AuthorizedDapp.sampleStokenet()
            ).asList(),
            resourcePreferences = ResourceAppPreferences.sample().asList(),
            mfaFactorInstances = MfaFactorInstances.sample().asList()
        )

        override fun other(): ProfileNetwork = ProfileNetwork(
            id = NetworkId.STOKENET,
            accounts = Accounts(
                Account.sampleStokenet.other(),
            ).asList(),
            personas = Personas(
                Persona.sampleStokenet.other()
            ).asList(),
            authorizedDapps = AuthorizedDapps(
                AuthorizedDapp.sampleStokenet.other()
            ).asList(),
            resourcePreferences = ResourceAppPreferences.sample.other().asList(),
            mfaFactorInstances = MfaFactorInstances.sample().asList()
        )
    }