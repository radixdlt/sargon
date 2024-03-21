package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.all
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class AppearanceIdTest: SampleTestable<AppearanceId> {

    override val samples: List<Sample<AppearanceId>>
        get() = listOf(AppearanceId.sample)

    @Test
    fun test() {
        assertThrows<CommonException.InvalidAppearanceId> {
            AppearanceId.init(validating = 12.toUByte())
        }

        assertEquals(
            List(12) {
                AppearanceId.init(validating = it.toUByte())
            },
            AppearanceId.all()
        )
    }

}