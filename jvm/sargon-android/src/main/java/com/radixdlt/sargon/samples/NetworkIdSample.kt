package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.NetworkId

@VisibleForTesting
val NetworkId.Companion.sample: Sample<NetworkId>
    get() = object : Sample<NetworkId> {

        override fun invoke(): NetworkId = NetworkId.MAINNET

        override fun other(): NetworkId = NetworkId.STOKENET
    }

class NetworkIdPreviewParameterProvider : PreviewParameterProvider<NetworkId> {
    override val values: Sequence<NetworkId>
        get() = NetworkId.sample.all.asSequence()
}