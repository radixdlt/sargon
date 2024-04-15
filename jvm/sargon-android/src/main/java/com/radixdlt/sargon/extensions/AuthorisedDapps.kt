package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.AuthorizedDapps
import com.radixdlt.sargon.authorizedDappsGetElements
import com.radixdlt.sargon.newAuthorizedDapps

@Throws(SargonException::class)
fun AuthorizedDapps.Companion.init(vararg authorisedDapp: AuthorizedDapp): AuthorizedDapps =
    newAuthorizedDapps(authorizedDapps = authorisedDapp.asList())

@Throws(SargonException::class)
fun AuthorizedDapps.Companion.init(authorisedDapps: List<AuthorizedDapp>): AuthorizedDapps =
    newAuthorizedDapps(authorizedDapps = authorisedDapps)

operator fun AuthorizedDapps.invoke() = authorizedDappsGetElements(authorizedDapps = this)

operator fun AuthorizedDapps.get(index: Int) = invoke().get(index = index)

operator fun AuthorizedDapps.contains(element: AuthorizedDapp) = invoke().contains(element = element)

val AuthorizedDapps.size: Int
    get() = invoke().size