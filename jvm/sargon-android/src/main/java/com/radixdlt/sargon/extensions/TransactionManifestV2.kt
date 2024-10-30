package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.ManifestSummary
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.TransactionManifestV2
import com.radixdlt.sargon.transactionManifestBlobsV2
import com.radixdlt.sargon.transactionManifestStringV2
import com.radixdlt.sargon.transactionManifestInvolvedPoolAddressesV2
import com.radixdlt.sargon.transactionManifestInvolvedResourceAddressesV2
import com.radixdlt.sargon.transactionManifestNetworkIdV2
import com.radixdlt.sargon.transactionManifestSummaryV2

val TransactionManifestV2.manifestString: String
    get() = transactionManifestStringV2(manifest = this)

val TransactionManifestV2.blobs: Blobs
    get() = transactionManifestBlobsV2(manifest = this)

val TransactionManifestV2.involvedPoolAddresses: List<PoolAddress>
    get() = transactionManifestInvolvedPoolAddressesV2(manifest = this)

val TransactionManifestV2.involvedResourceAddresses: List<ResourceAddress>
    get() = transactionManifestInvolvedResourceAddressesV2(manifest = this)

val TransactionManifestV2.summary: ManifestSummary?
    get() = transactionManifestSummaryV2(manifest = this)