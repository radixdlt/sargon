package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class RolaChallengeTest: SampleTestable<RolaChallenge> {
    override val samples: List<Sample<RolaChallenge>>
        get() = listOf(RolaChallenge.sample)

    @Test
    fun testHash() {

        assertEquals(
            "6fc75ec1d5c00941dc587c0a07409da1740c423c337c323ba7bdf68d61d4dd8e",
            RolaChallenge.sample().hash().hex,
        )
    }
}