package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.getBy
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.removeByAddress
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class PersonasTest: SampleTestable<Personas> {
    override val samples: List<Sample<Personas>>
        get() = listOf(Personas.sampleMainnet, Personas.sampleStokenet)

    @Test
    fun testListMethods() {
        val sample = Persona.sampleMainnet()
        val sampleOther = Persona.sampleMainnet.other()

        var list = Personas.init(sample)

        assertTrue(sample in list)
        assertEquals(1, list.size)
        assertEquals(sample, list[0])

        list = list.append(sampleOther)
        assertTrue(sampleOther in list)
        assertEquals(2, list.size)
        assertEquals(sampleOther, list[1])

        list = list.remove(sampleOther)
        Assertions.assertFalse(sampleOther in list)
        assertEquals(1, list.size)

        assertEquals(sample, list.getBy(sample.address))
        assertTrue(list.removeByAddress(sample.address).size == 0)
    }
}