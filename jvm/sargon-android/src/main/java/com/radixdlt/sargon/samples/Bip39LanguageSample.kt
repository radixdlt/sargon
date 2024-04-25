package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Bip39Language
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newBip39LanguageSample
import com.radixdlt.sargon.newBip39LanguageSampleOther

@UsesSampleValues
val Bip39Language.Companion.sample: Sample<Bip39Language>
    get() = object : Sample<Bip39Language> {
        override fun invoke(): Bip39Language = newBip39LanguageSample()

        override fun other(): Bip39Language = newBip39LanguageSampleOther()

    }