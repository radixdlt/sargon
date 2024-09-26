package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.dataUrlType
import com.radixdlt.sargon.extensions.urlExtension
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class VectorImageTypeTest: SampleTestable<VectorImageType> {
    override val samples: List<Sample<VectorImageType>>
        get() = listOf(VectorImageType.sample)

    @Test
    fun testExtension() {
        assertEquals(".svg", VectorImageType.sample().urlExtension)
        assertEquals(".pdf", VectorImageType.sample.other().urlExtension)
    }

    @Test
    fun testDataUrlType() {
        assertEquals("svg+xml", VectorImageType.sample().dataUrlType)
        assertEquals("pdf", VectorImageType.sample.other().dataUrlType)
    }
}