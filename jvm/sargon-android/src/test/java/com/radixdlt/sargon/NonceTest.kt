package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.random
import com.radixdlt.sargon.extensions.value
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonceTest: SampleTestable<Nonce> {
    override val samples: List<Sample<Nonce>>
        get() = listOf(Nonce.sample)

    @Test
    fun test_random() {
        val n = 10
        val nonceSet = List(n) {
            Nonce.random()
        }.toSet()

        assertEquals(n, nonceSet.size)
        assertEquals(n, nonceSet.map { it.value }.size)
    }
}