package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl

class AuthorizedDapps private constructor(
    array: IdentifiedArray<AccountAddress, AuthorizedDapp>
) : IdentifiedArray<AccountAddress, AuthorizedDapp> by array {

    constructor(authorizedDapps: List<AuthorizedDapp>) : this(
        IdentifiedArrayImpl(
            elements = authorizedDapps,
            identifier = { it.dappDefinitionAddress }
        )
    )

    constructor(
        vararg authorizedDapp: AuthorizedDapp
    ) : this(authorizedDapps = authorizedDapp.asList())
}