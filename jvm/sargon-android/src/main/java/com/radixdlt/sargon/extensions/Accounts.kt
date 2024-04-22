package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.Accounts
import com.radixdlt.sargon.accountsElementCount
import com.radixdlt.sargon.accountsGetAccountById
import com.radixdlt.sargon.accountsGetElements
import com.radixdlt.sargon.newAccounts
import com.radixdlt.sargon.newAccountsByAppending
import com.radixdlt.sargon.newAccountsByUpdatingOrAppending
import com.radixdlt.sargon.newAccountsByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newAccountsRemovedById
import com.radixdlt.sargon.newAccountsRemovedElement

fun Accounts.Companion.init(vararg account: Account): Accounts =
    newAccounts(accounts = account.asList())

fun Accounts.Companion.init(accounts: List<Account>): Accounts = newAccounts(accounts = accounts)

operator fun Accounts.invoke() = accountsGetElements(accounts = this)

operator fun Accounts.get(index: Int) = invoke().get(index = index)

operator fun Accounts.contains(element: Account) = invoke().contains(element = element)

val Accounts.size: Int
    get() = accountsElementCount(accounts = this).toInt()

fun Accounts.append(account: Account): Accounts =
    newAccountsByAppending(account = account, to = this)

fun Accounts.updateOrInsert(account: Account, index: Int): Accounts =
    newAccountsByUpdatingOrInsertingAtIndex(account = account, to = this, index = index.toULong())

fun Accounts.updateOrAppend(account: Account): Accounts =
    newAccountsByUpdatingOrAppending(account = account, to = this)

fun Accounts.removeByAddress(address: AccountAddress): Accounts =
    newAccountsRemovedById(idOfAccount = address, from = this)

fun Accounts.remove(account: Account): Accounts =
    newAccountsRemovedElement(account = account, from = this)

fun Accounts.getBy(address: AccountAddress): Account? =
    accountsGetAccountById(accounts = this, id = address)

