package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AccountAddress

class Accounts private constructor(
    array: IdentifiedArray<AccountAddress, Account>
) : IdentifiedArray<AccountAddress, Account> by array {

    constructor(accounts: List<Account>) : this(
        IdentifiedArrayImpl(
            elements = accounts,
            identifier = { it.address }
        )
    )

    constructor(vararg account: Account) : this(accounts = account.asList())
}

