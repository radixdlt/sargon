package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AuthorizedPersonaSimple
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.ReferencesToAuthorizedPersonas
import com.radixdlt.sargon.accountsElementCount
import com.radixdlt.sargon.accountsGetAccountById
import com.radixdlt.sargon.accountsGetElements
import com.radixdlt.sargon.newAccountsByAppending
import com.radixdlt.sargon.newAccountsByUpdatingOrAppending
import com.radixdlt.sargon.newAccountsByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newAccountsRemovedById
import com.radixdlt.sargon.newAccountsRemovedElement
import com.radixdlt.sargon.newReferencesToAuthorizedPersonas
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasByAppending
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasByUpdatingOrAppending
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasRemovedById
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasRemovedElement
import com.radixdlt.sargon.referencesToAuthorizedPersonasElementCount
import com.radixdlt.sargon.referencesToAuthorizedPersonasGetAuthorizedPersonaSimpleById
import com.radixdlt.sargon.referencesToAuthorizedPersonasGetElements

fun ReferencesToAuthorizedPersonas.Companion.init(vararg authorizedPersonaSimple: AuthorizedPersonaSimple): ReferencesToAuthorizedPersonas =
    init(referencesToAuthorizedPersonas = authorizedPersonaSimple.asList())

fun ReferencesToAuthorizedPersonas.Companion.init(referencesToAuthorizedPersonas: List<AuthorizedPersonaSimple>): ReferencesToAuthorizedPersonas =
    newReferencesToAuthorizedPersonas(referencesToAuthorizedPersonas = referencesToAuthorizedPersonas)

operator fun ReferencesToAuthorizedPersonas.invoke() = referencesToAuthorizedPersonasGetElements(
    referencesToAuthorizedPersonas = this
)

operator fun ReferencesToAuthorizedPersonas.get(index: Int) = invoke().get(index = index)

operator fun ReferencesToAuthorizedPersonas.contains(element: AuthorizedPersonaSimple) =
    invoke().contains(element = element)

val ReferencesToAuthorizedPersonas.size: Int
    get() = referencesToAuthorizedPersonasElementCount(
        referencesToAuthorizedPersonas = this
    ).toInt()

fun ReferencesToAuthorizedPersonas.append(
    authorizedPersonaSimple: AuthorizedPersonaSimple
): ReferencesToAuthorizedPersonas = newReferencesToAuthorizedPersonasByAppending(
    authorizedPersonaSimple = authorizedPersonaSimple,
    to = this
)

fun ReferencesToAuthorizedPersonas.updateOrInsert(
    authorizedPersonaSimple: AuthorizedPersonaSimple,
    index: Int
): ReferencesToAuthorizedPersonas = newReferencesToAuthorizedPersonasByUpdatingOrInsertingAtIndex(
    authorizedPersonaSimple = authorizedPersonaSimple,
    to = this,
    index = index.toULong()
)

fun ReferencesToAuthorizedPersonas.updateOrAppend(
    authorizedPersonaSimple: AuthorizedPersonaSimple
): ReferencesToAuthorizedPersonas = newReferencesToAuthorizedPersonasByUpdatingOrAppending(
    authorizedPersonaSimple = authorizedPersonaSimple,
    to = this
)

fun ReferencesToAuthorizedPersonas.removeByAddress(
    address: IdentityAddress
): ReferencesToAuthorizedPersonas = newReferencesToAuthorizedPersonasRemovedById(
    idOfAuthorizedPersonaSimple = address,
    from = this
)

fun ReferencesToAuthorizedPersonas.remove(
    authorizedPersonaSimple: AuthorizedPersonaSimple
): ReferencesToAuthorizedPersonas = newReferencesToAuthorizedPersonasRemovedElement(
    authorizedPersonaSimple = authorizedPersonaSimple,
    from = this
)

fun ReferencesToAuthorizedPersonas.getBy(
    address: IdentityAddress
): AuthorizedPersonaSimple? = referencesToAuthorizedPersonasGetAuthorizedPersonaSimpleById(
    referencesToAuthorizedPersonas = this,
    id = address
)