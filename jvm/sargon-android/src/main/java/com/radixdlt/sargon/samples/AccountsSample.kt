package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.Accounts
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val Accounts.Companion.sampleMainnet: Sample<Accounts>
    get() = object : Sample<Accounts> {
        override fun invoke(): Accounts = Accounts.init(
            listOf(
                Account.sampleMainnet.alice,
                Account.sampleMainnet.bob
            )
        )

        override fun other(): Accounts = Accounts.init(
            listOf(Account.sampleMainnet.carol)
        )
    }

@UsesSampleValues
val Accounts.Companion.sampleStokenet: Sample<Accounts>
    get() = object : Sample<Accounts> {
        override fun invoke(): Accounts = Accounts.init(
            listOf(
                Account.sampleStokenet.nadia,
                Account.sampleStokenet.olivia
            )
        )

        override fun other(): Accounts = Accounts.init(
            listOf(Account.sampleStokenet.paige)
        )
    }