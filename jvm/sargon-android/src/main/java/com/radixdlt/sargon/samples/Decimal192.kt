package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Decimal192
import com.radixdlt.sargon.extensions.MAX
import com.radixdlt.sargon.extensions.toDecimal192

@VisibleForTesting
val Decimal192.Companion.sample: Sample<Decimal192>
    get() = object : Sample<Decimal192> {

        override fun invoke(): Decimal192 = 123456789.toDecimal192()

        override fun other(): Decimal192 = Decimal192.MAX
    }

class Decimal192PreviewParameterProvider : PreviewParameterProvider<Decimal192> {
    override val values: Sequence<Decimal192>
        get() = Decimal192.sample.all.asSequence()
}