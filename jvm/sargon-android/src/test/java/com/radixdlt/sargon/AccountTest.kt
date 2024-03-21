package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet

class AccountTest: SampleTestable<Account> {
    override val samples: List<Sample<Account>>
        get() = listOf(Account.sampleMainnet, Account.sampleStokenet)
}