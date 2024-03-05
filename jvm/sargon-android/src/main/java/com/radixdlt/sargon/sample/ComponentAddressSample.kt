package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.newComponentAddressSample
import com.radixdlt.sargon.newComponentAddressSampleOther

@VisibleForTesting
val ComponentAddress.Companion.sample: Sample<ComponentAddress>
    get() = object : Sample<ComponentAddress> {

        override fun invoke(): ComponentAddress = newComponentAddressSample()

        override fun other(): ComponentAddress = newComponentAddressSampleOther()
    }

class ComponentAddressPreviewParameterProvider: PreviewParameterProvider<ComponentAddress> {
    override val values: Sequence<ComponentAddress>
        get() = ComponentAddress.sample.all.asSequence()

}