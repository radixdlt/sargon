package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.newComponentAddressSampleMainnetGlobal
import com.radixdlt.sargon.newComponentAddressSampleMainnetInternal
import com.radixdlt.sargon.newComponentAddressSampleStokenetGlobal
import com.radixdlt.sargon.newComponentAddressSampleStokenetInternal

@VisibleForTesting
val ComponentAddress.Companion.sampleMainnet: Sample<ComponentAddress>
    get() = object : Sample<ComponentAddress> {

        override fun invoke(): ComponentAddress = newComponentAddressSampleMainnetGlobal()

        override fun other(): ComponentAddress = newComponentAddressSampleMainnetInternal()
    }

@VisibleForTesting
val ComponentAddress.Companion.sampleStokenet: Sample<ComponentAddress>
    get() = object : Sample<ComponentAddress> {

        override fun invoke(): ComponentAddress = newComponentAddressSampleStokenetGlobal()

        override fun other(): ComponentAddress = newComponentAddressSampleStokenetInternal()
    }