package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.blobs
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.involvedPoolAddresses
import com.radixdlt.sargon.extensions.involvedResourceAddresses
import com.radixdlt.sargon.extensions.manifestString
import com.radixdlt.sargon.extensions.summary
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class SubintentManifestTest : SampleTestable<SubintentManifest> {

    override val samples: List<Sample<SubintentManifest>>
        get() = listOf(SubintentManifest.sample)

    @Test
    fun test() {
        val manifest = SubintentManifest.sample()

        assertTrue(manifest.manifestString.contains("CALL_METHOD"))
        assertEquals(Blobs.init(emptyList()), manifest.blobs)
        assertEquals(emptyList<PoolAddress>(), manifest.involvedPoolAddresses)
        assertEquals(listOf(ResourceAddress.sampleMainnet()), manifest.involvedResourceAddresses)
    }

    @Test
    fun testSummary() {
        val summary = SubintentManifest.sample().summary

        assertTrue(summary.accountWithdrawals.isNotEmpty())
    }
}