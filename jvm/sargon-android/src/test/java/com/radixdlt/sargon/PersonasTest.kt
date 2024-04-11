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

class PersonasTest: SampleTestable<Personas> {
    override val samples: List<Sample<Personas>>
        get() = listOf(Personas.sampleMainnet, Personas.sampleStokenet)

    @Test
    fun testListMethods() {
        val first = Persona.sampleMainnet()
        val samples = Personas.init(first)

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