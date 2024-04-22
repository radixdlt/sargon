package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.AuthorizedDapps
import com.radixdlt.sargon.authorizedDappsGetAuthorizedDappById
import com.radixdlt.sargon.authorizedDappsGetElements
import com.radixdlt.sargon.newAccountsByUpdatingOrAppending
import com.radixdlt.sargon.newAuthorizedDapps
import com.radixdlt.sargon.newAuthorizedDappsByAppending
import com.radixdlt.sargon.newAuthorizedDappsByUpdatingOrAppending
import com.radixdlt.sargon.newAuthorizedDappsByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newAuthorizedDappsRemovedById
import com.radixdlt.sargon.newAuthorizedDappsRemovedElement

@Throws(SargonException::class)
fun AuthorizedDapps.Companion.init(vararg authorisedDapp: AuthorizedDapp): AuthorizedDapps =
    newAuthorizedDapps(authorizedDapps = authorisedDapp.asList())

@Throws(SargonException::class)
fun AuthorizedDapps.Companion.init(authorisedDapps: List<AuthorizedDapp>): AuthorizedDapps =
    newAuthorizedDapps(authorizedDapps = authorisedDapps)

operator fun AuthorizedDapps.invoke() = authorizedDappsGetElements(authorizedDapps = this)

operator fun AuthorizedDapps.get(index: Int) = invoke().get(index = index)

operator fun AuthorizedDapps.contains(element: AuthorizedDapp) =
    invoke().contains(element = element)

val AuthorizedDapps.size: Int
    get() = invoke().size

fun AuthorizedDapps.append(authorizedDApp: AuthorizedDapp): AuthorizedDapps =
    newAuthorizedDappsByAppending(authorizedDapp = authorizedDApp, to = this)

fun AuthorizedDapps.updateOrInsert(authorizedDApp: AuthorizedDapp, index: Int): AuthorizedDapps =
    newAuthorizedDappsByUpdatingOrInsertingAtIndex(
        authorizedDapp = authorizedDApp,
        to = this,
        index = index.toULong()
    )

fun AuthorizedDapps.updateOrAppend(authorizedDApp: AuthorizedDapp): AuthorizedDapps =
    newAuthorizedDappsByUpdatingOrAppending(authorizedDapp = authorizedDApp, to = this)

fun AuthorizedDapps.removeByAddress(address: AccountAddress): AuthorizedDapps =
    newAuthorizedDappsRemovedById(idOfAuthorizedDapp = address, from = this)

fun AuthorizedDapps.remove(authorizedDApp: AuthorizedDapp): AuthorizedDapps =
    newAuthorizedDappsRemovedElement(authorizedDapp = authorizedDApp, from = this)

fun AuthorizedDapps.getBy(address: AccountAddress): AuthorizedDapp? =
    authorizedDappsGetAuthorizedDappById(authorizedDapps = this, id = address)