package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.ManifestSummary
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.SubintentManifest
import com.radixdlt.sargon.subintentManifestBlobs
import com.radixdlt.sargon.subintentManifestInvolvedPoolAddresses
import com.radixdlt.sargon.subintentManifestInvolvedResourceAddresses
import com.radixdlt.sargon.subintentManifestNetworkId
import com.radixdlt.sargon.subintentManifestString
import com.radixdlt.sargon.subintentManifestSummary

val SubintentManifest.manifestString: String
    get() = subintentManifestString(manifest = this)

val SubintentManifest.blobs: Blobs
    get() = subintentManifestBlobs(manifest = this)

val SubintentManifest.involvedPoolAddresses: List<PoolAddress>
    get() = subintentManifestInvolvedPoolAddresses(manifest = this)

val SubintentManifest.involvedResourceAddresses: List<ResourceAddress>
    get() = subintentManifestInvolvedResourceAddresses(manifest = this)