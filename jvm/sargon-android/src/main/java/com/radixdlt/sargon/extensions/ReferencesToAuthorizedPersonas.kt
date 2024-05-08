package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AuthorizedPersonaSimple
import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl
import com.radixdlt.sargon.IdentityAddress

class ReferencesToAuthorizedPersonas private constructor(
    array: IdentifiedArray<IdentityAddress, AuthorizedPersonaSimple>
) : IdentifiedArray<IdentityAddress, AuthorizedPersonaSimple> by array {

    constructor(authorizedPersonasSimple: List<AuthorizedPersonaSimple>) : this(
        IdentifiedArrayImpl(
            elements = authorizedPersonasSimple,
            identifier = { it.identityAddress }
        )
    )

    constructor(vararg authorizedPersonaSimple: AuthorizedPersonaSimple) : this(
        IdentifiedArrayImpl(element = authorizedPersonaSimple, identifier = { it.identityAddress })
    )
}