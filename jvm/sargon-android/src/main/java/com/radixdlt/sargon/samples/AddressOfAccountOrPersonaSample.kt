package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AddressOfAccountOrPersona
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleMainnet
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleMainnetOther
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleStokenet
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleStokenetOther

@VisibleForTesting
val AddressOfAccountOrPersona.Companion.sampleMainnet: Sample<AddressOfAccountOrPersona>
    get() = object : Sample<AddressOfAccountOrPersona> {
        override fun invoke(): AddressOfAccountOrPersona = newAddressOfAccountOrPersonaSampleMainnet()

        override fun other(): AddressOfAccountOrPersona = newAddressOfAccountOrPersonaSampleMainnetOther()
    }

class AddressOfAccountOrPersonaMainnetAddressPreviewParameterProvider :
    PreviewParameterProvider<AddressOfAccountOrPersona> {
    override val values: Sequence<AddressOfAccountOrPersona>
        get() = AddressOfAccountOrPersona.sampleMainnet.all.asSequence()

}

@VisibleForTesting
val AddressOfAccountOrPersona.Companion.sampleStokenet: Sample<AddressOfAccountOrPersona>
    get() = object : Sample<AddressOfAccountOrPersona> {
        override fun invoke(): AddressOfAccountOrPersona = newAddressOfAccountOrPersonaSampleStokenet()

        override fun other(): AddressOfAccountOrPersona = newAddressOfAccountOrPersonaSampleStokenetOther()
    }

class AddressOfAccountOrPersonaStokenetAddressPreviewParameterProvider :
    PreviewParameterProvider<AddressOfAccountOrPersona> {
    override val values: Sequence<AddressOfAccountOrPersona>
        get() = AddressOfAccountOrPersona.sampleStokenet.all.asSequence()

}