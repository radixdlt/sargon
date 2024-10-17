package com.radixdlt.sargon

import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.extensions.blobs
import com.radixdlt.sargon.extensions.manifestString
import com.radixdlt.sargon.extensions.involvedPoolAddresses
import com.radixdlt.sargon.extensions.involvedResourceAddresses
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.summary
import com.radixdlt.sargon.extensions.toList
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class TransactionManifestV2Test : SampleTestable<TransactionManifestV2> {

    override val samples: List<Sample<TransactionManifestV2>>
        get() = listOf(TransactionManifestV2.sample)

    @Test
    fun test_manifest_string() {
        val manifest = TransactionManifestV2.sample()
        assertTrue(manifest.manifestString.contains("CALL_METHOD"))
    }

    @Test
    fun test_network_id() {
        assertEquals(
            NetworkId.MAINNET,
            TransactionManifestV2.sample().networkId
        )
    }

    @Test
    fun test_blobs() {
        assertEquals(
            emptyList<Blob>(),
            TransactionManifestV2.sample().blobs.toList()
        )
    }

    @Test
    fun test_involved_resource_addresses() {
        assertEquals(
            listOf(ResourceAddress.sampleMainnet.xrd),
            TransactionManifestV2.sample().involvedResourceAddresses
        )
    }

    @Test
    fun test_involved_pool_addresses() {
        assertEquals(
            emptyList<PoolAddress>(),
            TransactionManifestV2.sample().involvedPoolAddresses
        )
    }

    @Test
    fun test_summary() {
        val summary = TransactionManifestV2.sample().summary
        assertEquals(
            listOf(AccountAddress.sampleMainnet()),
            summary?.addressesOfAccountsWithdrawnFrom
        )
    }
}