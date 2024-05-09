package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.annotation.KoverIgnore

class AuthorizedDapps private constructor(
    private val array: IdentifiedArray<AccountAddress, AuthorizedDapp>
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

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as AuthorizedDapps

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "AuthorizedDapps(array=$array)"
    }

}

fun List<AuthorizedDapp>.asIdentifiable() = AuthorizedDapps(authorizedDapps = this)
