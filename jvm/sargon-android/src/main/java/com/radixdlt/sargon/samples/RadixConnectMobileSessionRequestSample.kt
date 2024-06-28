package com.radixdlt.sargon.samples

import com.radixdlt.sargon.RadixConnectMobileSessionRequest
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newRadixConnectMobileSessionRequestSample
import com.radixdlt.sargon.newRadixConnectMobileSessionRequestSampleOther

@UsesSampleValues
val RadixConnectMobileSessionRequest.Companion.sample: Sample<RadixConnectMobileSessionRequest>
    get() = object : Sample<RadixConnectMobileSessionRequest> {

        override fun invoke(): RadixConnectMobileSessionRequest =
            newRadixConnectMobileSessionRequestSample()

        override fun other(): RadixConnectMobileSessionRequest =
            newRadixConnectMobileSessionRequestSampleOther()
    }