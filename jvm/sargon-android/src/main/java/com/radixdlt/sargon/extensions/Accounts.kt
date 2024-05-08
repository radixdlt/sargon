package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl

class Accounts private constructor(
    array: IdentifiedArray<AccountAddress, Account>
) : IdentifiedArray<AccountAddress, Account> by array {

    constructor(accounts: List<Account>) : this(
        IdentifiedArrayImpl(
            elements = accounts,
            identifier = { it.address }
        )
    )

    constructor(vararg account: Account) : this(
        IdentifiedArrayImpl(element = account, identifier = { it.address })
    )
}

