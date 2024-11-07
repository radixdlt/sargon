package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.random
import com.radixdlt.sargon.extensions.value
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class IntentDiscriminatorTest : SampleTestable<IntentDiscriminator> {

    override val samples: List<Sample<IntentDiscriminator>>
        get() = listOf(IntentDiscriminator.sample)

    @Test
    fun test_random() {
        val n = 10
        val discriminators = List(n) {
            IntentDiscriminator.random()
        }.toSet()

        assertEquals(n, discriminators.size)
        assertEquals(n, discriminators.map { it.value }.size)
    }

}