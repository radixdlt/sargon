package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.newAccountAddressSampleMainnet
import com.radixdlt.sargon.newAccountAddressSampleMainnetOther
import com.radixdlt.sargon.newAccountAddressSampleStokenet
import com.radixdlt.sargon.newAccountAddressSampleStokenetOther

@VisibleForTesting
val AccountAddress.Companion.sampleMainnet: Sample<AccountAddress>
    get() = object : Sample<AccountAddress> {

        override fun invoke(): AccountAddress = newAccountAddressSampleMainnet()

        override fun other(): AccountAddress = newAccountAddressSampleMainnetOther()

    }

@VisibleForTesting
val AccountAddress.Companion.sampleStokenet: Sample<AccountAddress>
    get() = object : Sample<AccountAddress> {

        override fun invoke(): AccountAddress = newAccountAddressSampleStokenet()

        override fun other(): AccountAddress = newAccountAddressSampleStokenetOther()
    }

class AccountAddressMainnetAddressPreviewParameterProvider :
    PreviewParameterProvider<AccountAddress> {
    override val values: Sequence<AccountAddress>
        get() = AccountAddress.sampleMainnet.all.asSequence()

}

class AccountAddressStokenetAddressPreviewParameterProvider :
    PreviewParameterProvider<AccountAddress> {
    override val values: Sequence<AccountAddress>
        get() = AccountAddress.sampleStokenet.all.asSequence()

}