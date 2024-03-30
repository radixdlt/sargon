package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.RadixConnectPassword
import com.radixdlt.sargon.newRadixConnectPasswordSample
import com.radixdlt.sargon.newRadixConnectPasswordSampleOther

@VisibleForTesting
val RadixConnectPassword.Companion.sample: Sample<RadixConnectPassword>
    get() = object : Sample<RadixConnectPassword> {

        override fun invoke(): RadixConnectPassword = newRadixConnectPasswordSample()

        override fun other(): RadixConnectPassword = newRadixConnectPasswordSampleOther()

    }