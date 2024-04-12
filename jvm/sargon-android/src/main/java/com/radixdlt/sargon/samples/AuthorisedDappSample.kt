package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSampleAuthorisedDappMainnetDashboard
import com.radixdlt.sargon.newSampleAuthorisedDappMainnetGumballclub
import com.radixdlt.sargon.newSampleAuthorisedDappStokenetDevconsole
import com.radixdlt.sargon.newSampleAuthorisedDappStokenetSandbox

@UsesSampleValues
object AuthorisedDappSampleMainnet: Sample<AuthorizedDapp> {
    override fun invoke(): AuthorizedDapp = dashboard

    override fun other(): AuthorizedDapp = gumballClub

    val dashboard: AuthorizedDapp
        get() = newSampleAuthorisedDappMainnetDashboard()

    val gumballClub: AuthorizedDapp
        get() = newSampleAuthorisedDappMainnetGumballclub()
}

@UsesSampleValues
val AuthorizedDapp.Companion.sampleMainnet: AuthorisedDappSampleMainnet
    get() = AuthorisedDappSampleMainnet

@UsesSampleValues
object AuthorisedDappSampleStokenet: Sample<AuthorizedDapp> {
    override fun invoke(): AuthorizedDapp = devConsole

    override fun other(): AuthorizedDapp = sandbox

    val devConsole: AuthorizedDapp
        get() = newSampleAuthorisedDappStokenetDevconsole()

    val sandbox: AuthorizedDapp
        get() = newSampleAuthorisedDappStokenetSandbox()
}

@UsesSampleValues
val AuthorizedDapp.Companion.sampleStokenet: AuthorisedDappSampleStokenet
    get() = AuthorisedDappSampleStokenet
