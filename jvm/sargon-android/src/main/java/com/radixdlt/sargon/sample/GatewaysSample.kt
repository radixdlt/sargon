package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.newGatewaysSample
import com.radixdlt.sargon.newGatewaysSampleOther

@VisibleForTesting
val Gateways.Companion.sample: Sample<Gateways>
    get() = object : Sample<Gateways> {

        override fun invoke(): Gateways = newGatewaysSample()

        override fun other(): Gateways = newGatewaysSampleOther()
    }

class GatewaysPreviewParameterProvider : PreviewParameterProvider<Gateways> {
    override val values: Sequence<Gateways>
        get() = Gateways.sample.all.asSequence()
}