package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.RadixConnectPassword
import com.radixdlt.sargon.newRadixConnectPasswordSample
import com.radixdlt.sargon.newRadixConnectPasswordSampleOther

@UsesSampleValues
val RadixConnectPassword.Companion.sample: Sample<RadixConnectPassword>
    get() = object : Sample<RadixConnectPassword> {

        override fun invoke(): RadixConnectPassword = newRadixConnectPasswordSample()

        override fun other(): RadixConnectPassword = newRadixConnectPasswordSampleOther()

    }