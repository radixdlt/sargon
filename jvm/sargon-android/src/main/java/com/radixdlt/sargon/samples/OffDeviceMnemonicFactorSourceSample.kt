package com.radixdlt.sargon.samples

import com.radixdlt.sargon.OffDeviceMnemonicFactorSource
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newOffDeviceMnemonicFactorSourceSample
import com.radixdlt.sargon.newOffDeviceMnemonicFactorSourceSampleOther

@UsesSampleValues
val OffDeviceMnemonicFactorSource.Companion.sample: Sample<OffDeviceMnemonicFactorSource>
    get() = object : Sample<OffDeviceMnemonicFactorSource> {
        override fun invoke(): OffDeviceMnemonicFactorSource = newOffDeviceMnemonicFactorSourceSample()

        override fun other(): OffDeviceMnemonicFactorSource = newOffDeviceMnemonicFactorSourceSampleOther()

    }