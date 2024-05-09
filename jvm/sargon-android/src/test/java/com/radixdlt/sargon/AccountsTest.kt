package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Accounts
import com.radixdlt.sargon.samples.sampleMainnet

internal class AccountsTest: IdentifiedArrayTest<Accounts, AccountAddress, Account>() {

    override fun element(): Account = Account.sampleMainnet()

    override fun elementWithDifferentId(): Account = Account.sampleMainnet.other()

    override fun identifier(element: Account): AccountAddress = element.address

    override fun init(element: Account): Accounts = Accounts(element)

}