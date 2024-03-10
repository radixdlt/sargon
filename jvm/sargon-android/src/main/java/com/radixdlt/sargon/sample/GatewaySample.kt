package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.gatewayMainnet
import com.radixdlt.sargon.gatewayStokenet

@VisibleForTesting
val Gateway.Companion.sampleMainnet: Sample<Gateway>
    get() = object : Sample<Gateway> {

        override fun invoke(): Gateway = gatewayMainnet()

        override fun other(): Gateway = invoke()
    }

@VisibleForTesting
val Gateway.Companion.sampleStokenet: Sample<Gateway>
    get() = object : Sample<Gateway> {

        override fun invoke(): Gateway = gatewayStokenet()

        override fun other(): Gateway = invoke()
    }

class GatewayMainnetPreviewParameterProvider : PreviewParameterProvider<Gateway> {
    override val values: Sequence<Gateway>
        get() = Gateway.sampleMainnet.all.asSequence()
}

class GatewayStokenetPreviewParameterProvider : PreviewParameterProvider<Gateway> {
    override val values: Sequence<Gateway>
        get() = Gateway.sampleStokenet.all.asSequence()
}