package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.PerAssetTransfers
import com.radixdlt.sargon.TransactionManifest
import com.radixdlt.sargon.newPerAssetTransfersSample
import com.radixdlt.sargon.newPerAssetTransfersSampleOther
import com.radixdlt.sargon.newTransactionManifestSample
import com.radixdlt.sargon.newTransactionManifestSampleOther

@VisibleForTesting
val TransactionManifest.Companion.sample: Sample<TransactionManifest>
    get() = object : Sample<TransactionManifest> {

        override fun invoke(): TransactionManifest = newTransactionManifestSample()

        override fun other(): TransactionManifest = newTransactionManifestSampleOther()

    }

@VisibleForTesting
val PerAssetTransfers.Companion.sample: Sample<PerAssetTransfers>
    get() = object : Sample<PerAssetTransfers> {

        override fun invoke(): PerAssetTransfers = newPerAssetTransfersSample()

        override fun other(): PerAssetTransfers = newPerAssetTransfersSampleOther()

    }