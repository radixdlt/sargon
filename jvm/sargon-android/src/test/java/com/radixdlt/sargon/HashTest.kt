package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class HashTest: SampleTestable<Hash> {

    override val samples: List<Sample<Hash>>
        get() = listOf(Hash.sample)

    @Test
    fun test() {
        val hash = Hash.sample()

        assertEquals(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935",
            hash.hex
        )
    }

}