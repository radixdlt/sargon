package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.Accounts
import com.radixdlt.sargon.getAccounts
import com.radixdlt.sargon.newAccounts

fun Accounts.Companion.init(vararg account: Account): Accounts =
    newAccounts(accounts = account.asList())

fun Accounts.Companion.init(accounts: List<Account>): Accounts = newAccounts(accounts = accounts)

operator fun Accounts.invoke() = getAccounts(accounts = this)

operator fun Accounts.get(index: Int) = invoke().get(index = index)

operator fun Accounts.contains(element: Account) = invoke().contains(element = element)

val Accounts.size: Int
    get() = invoke().size