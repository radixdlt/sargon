package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.discriminant
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Cap26EntityKindTest: SampleTestable<Cap26EntityKind> {
    override val samples: List<Sample<Cap26EntityKind>>
        get() = listOf(Cap26EntityKind.sample)

    @Test
    fun testDiscriminant() {
        assertEquals(
            "Account",
            Cap26EntityKind.sample().discriminant
        )

        assertEquals(
            "Identity",
            Cap26EntityKind.sample.other().discriminant
        )
    }
}