package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
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
            Hash.init(hash.hex),
            hash
        )

        assertEquals(
            Hash.init(hash.bytes),
            hash
        )
    }

}