package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.TransactionManifestV2
import com.radixdlt.sargon.newTransactionManifestV2Sample
import com.radixdlt.sargon.newTransactionManifestV2SampleOther

@UsesSampleValues
val TransactionManifestV2.Companion.sample: Sample<TransactionManifestV2>
    get() = object : Sample<TransactionManifestV2> {

        override fun invoke(): TransactionManifestV2 = newTransactionManifestV2Sample()

        override fun other(): TransactionManifestV2 = newTransactionManifestV2SampleOther()

    }