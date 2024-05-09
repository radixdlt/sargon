package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.annotation.KoverIgnore

class Accounts private constructor(
    private val array: IdentifiedArray<AccountAddress, Account>
) : IdentifiedArray<AccountAddress, Account> by array {

    constructor(accounts: List<Account>) : this(
        IdentifiedArrayImpl(
            elements = accounts,
            identifier = { it.address }
        )
    )

    constructor(vararg account: Account) : this(accounts = account.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Accounts

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "Accounts(array=$array)"
    }

}

fun List<Account>.asIdentifiable() = Accounts(accounts = this)
