package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.AuthorizedDapps
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val AuthorizedDapps.Companion.sampleMainnet: Sample<AuthorizedDapps>
    get() = object : Sample<AuthorizedDapps> {
        override fun invoke(): AuthorizedDapps = AuthorizedDapps.init(
            listOf(AuthorizedDapp.sampleMainnet.dashboard, AuthorizedDapp.sampleMainnet.gumballClub)
        )

        override fun other(): AuthorizedDapps = AuthorizedDapps.init(
            AuthorizedDapp.sampleMainnet.gumballClub
        )

    }

@UsesSampleValues
val AuthorizedDapps.Companion.sampleStokenet: Sample<AuthorizedDapps>
    get() = object : Sample<AuthorizedDapps> {
        override fun invoke(): AuthorizedDapps = AuthorizedDapps.init(
            listOf(AuthorizedDapp.sampleStokenet.devConsole, AuthorizedDapp.sampleStokenet.sandbox)
        )

        override fun other(): AuthorizedDapps = AuthorizedDapps.init(
            AuthorizedDapp.sampleStokenet.sandbox
        )

    }
