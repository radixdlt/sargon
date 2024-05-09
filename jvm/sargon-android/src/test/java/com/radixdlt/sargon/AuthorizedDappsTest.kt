package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.AuthorizedDapps
import com.radixdlt.sargon.samples.sampleMainnet

internal class AuthorizedDappsTest: IdentifiedArrayTest<AuthorizedDapps, AccountAddress, AuthorizedDapp>() {
    override fun element(): AuthorizedDapp = AuthorizedDapp.sampleMainnet()

    override fun elementWithDifferentId(): AuthorizedDapp = AuthorizedDapp.sampleMainnet.other()

    override fun identifier(element: AuthorizedDapp): AccountAddress = element.dappDefinitionAddress

    override fun init(element: AuthorizedDapp): AuthorizedDapps = AuthorizedDapps(element)

}