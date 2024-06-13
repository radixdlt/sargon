package com.radixdlt.sargon.samples

import com.radixdlt.sargon.RadixConnectMobileLinkRequest
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newRadixConnectMobileLinkRequestSample
import com.radixdlt.sargon.newRadixConnectMobileLinkRequestSampleOther

@UsesSampleValues
val RadixConnectMobileLinkRequest.Companion.sample: Sample<RadixConnectMobileLinkRequest>
    get() = object : Sample<RadixConnectMobileLinkRequest> {

        override fun invoke(): RadixConnectMobileLinkRequest =
            newRadixConnectMobileLinkRequestSample()

        override fun other(): RadixConnectMobileLinkRequest =
            newRadixConnectMobileLinkRequestSampleOther()
    }