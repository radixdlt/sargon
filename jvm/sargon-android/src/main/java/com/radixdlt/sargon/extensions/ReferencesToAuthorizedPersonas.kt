package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AuthorizedPersonaSimple
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.annotation.KoverIgnore

class ReferencesToAuthorizedPersonas private constructor(
    private val array: IdentifiedArray<IdentityAddress, AuthorizedPersonaSimple>
) : IdentifiedArray<IdentityAddress, AuthorizedPersonaSimple> by array {

    constructor(authorizedPersonasSimple: List<AuthorizedPersonaSimple>) : this(
        IdentifiedArrayImpl(
            elements = authorizedPersonasSimple,
            identifier = { it.identityAddress }
        )
    )

    constructor(vararg authorizedPersonaSimple: AuthorizedPersonaSimple) : this(
        authorizedPersonasSimple = authorizedPersonaSimple.asList()
    )

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ReferencesToAuthorizedPersonas

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "ReferencesToAuthorizedPersonas(array=$array)"
    }

}

fun List<AuthorizedPersonaSimple>.asIdentifiable() = ReferencesToAuthorizedPersonas(authorizedPersonasSimple = this)
