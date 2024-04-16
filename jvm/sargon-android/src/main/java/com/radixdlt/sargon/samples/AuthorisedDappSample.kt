package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAuthorizedDappSampleMainnetDashboard
import com.radixdlt.sargon.newAuthorizedDappSampleMainnetGumballclub
import com.radixdlt.sargon.newAuthorizedDappSampleStokenetDevconsole
import com.radixdlt.sargon.newAuthorizedDappSampleStokenetSandbox

@UsesSampleValues
object AuthorisedDappSampleMainnet: Sample<AuthorizedDapp> {
    override fun invoke(): AuthorizedDapp = dashboard

    override fun other(): AuthorizedDapp = gumballClub

    val dashboard: AuthorizedDapp
        get() = newAuthorizedDappSampleMainnetDashboard()

    val gumballClub: AuthorizedDapp
        get() = newAuthorizedDappSampleMainnetGumballclub()
}

@UsesSampleValues
val AuthorizedDapp.Companion.sampleMainnet: AuthorisedDappSampleMainnet
    get() = AuthorisedDappSampleMainnet

@UsesSampleValues
object AuthorisedDappSampleStokenet: Sample<AuthorizedDapp> {
    override fun invoke(): AuthorizedDapp = devConsole

    override fun other(): AuthorizedDapp = sandbox

    val devConsole: AuthorizedDapp
        get() = newAuthorizedDappSampleStokenetDevconsole()

    val sandbox: AuthorizedDapp
        get() = newAuthorizedDappSampleStokenetSandbox()
}

@UsesSampleValues
val AuthorizedDapp.Companion.sampleStokenet: AuthorisedDappSampleStokenet
    get() = AuthorisedDappSampleStokenet
