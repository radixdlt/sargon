package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AccountForDisplay
import com.radixdlt.sargon.newAccountForDisplayFromAccount

fun AccountForDisplay.Companion.from(account: Account) = newAccountForDisplayFromAccount(
    account = account
)