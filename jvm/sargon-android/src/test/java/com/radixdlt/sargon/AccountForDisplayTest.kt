package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.from
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AccountForDisplayTest: SampleTestable<AccountForDisplay> {
    override val samples: List<Sample<AccountForDisplay>>
        get() = listOf(AccountForDisplay.sample)

    @Test
    fun testFromAccount() {
        val account = Account.sampleMainnet()

        assertEquals(
            AccountForDisplay(
                address = account.address,
                displayName = account.displayName,
                appearanceId = account.appearanceId
            ),
            AccountForDisplay.from(account)
        )

    }
}