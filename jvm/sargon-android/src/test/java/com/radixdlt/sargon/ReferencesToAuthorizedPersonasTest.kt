package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ReferencesToAuthorizedPersonas
import com.radixdlt.sargon.samples.sampleMainnet

internal class ReferencesToAuthorizedPersonasTest :
    IdentifiedArrayTest<ReferencesToAuthorizedPersonas, IdentityAddress, AuthorizedPersonaSimple>() {
    override fun element(): AuthorizedPersonaSimple = AuthorizedPersonaSimple.sampleMainnet()

    override fun elementWithDifferentId(): AuthorizedPersonaSimple = AuthorizedPersonaSimple.sampleMainnet.other()

    override fun identifier(element: AuthorizedPersonaSimple): IdentityAddress =
        element.identityAddress

    override fun init(element: AuthorizedPersonaSimple): ReferencesToAuthorizedPersonas =
        ReferencesToAuthorizedPersonas(element)

}