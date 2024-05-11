package com.radixdlt.sargon.samples

import com.radixdlt.sargon.RadixConnectPurpose
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newRadixConnectPurposeSample
import com.radixdlt.sargon.newRadixConnectPurposeSampleOther

@UsesSampleValues
val RadixConnectPurpose.Companion.sample: Sample<RadixConnectPurpose>
    get() = object : Sample<RadixConnectPurpose> {

        override fun invoke(): RadixConnectPurpose = newRadixConnectPurposeSample()

        override fun other(): RadixConnectPurpose = newRadixConnectPurposeSampleOther()

    }