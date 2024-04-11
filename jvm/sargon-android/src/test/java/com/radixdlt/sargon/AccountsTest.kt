package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class AccountsTest: SampleTestable<Accounts> {
    override val samples: List<Sample<Accounts>>
        get() = listOf(Accounts.sampleMainnet, Accounts.sampleStokenet)

    @Test
    fun testListMethods() {
        val first = Account.sampleMainnet()
        val samples = Accounts.init(first)

        assertTrue(first in samples)
        assertEquals(
            1,
            samples.size
        )
        assertEquals(
            first,
            samples[0]
        )
    }
}