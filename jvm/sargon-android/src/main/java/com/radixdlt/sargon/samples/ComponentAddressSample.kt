package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.newComponentAddressSampleMainnetGlobal
import com.radixdlt.sargon.newComponentAddressSampleMainnetInternal

@VisibleForTesting
val ComponentAddress.Companion.sample: Sample<ComponentAddress>
    get() = object : Sample<ComponentAddress> {

        override fun invoke(): ComponentAddress = newComponentAddressSampleMainnetGlobal()

        override fun other(): ComponentAddress = newComponentAddressSampleMainnetInternal()
    }

class ComponentAddressPreviewParameterProvider: PreviewParameterProvider<ComponentAddress> {
    override val values: Sequence<ComponentAddress>
        get() = ComponentAddress.sample.all.asSequence()

}