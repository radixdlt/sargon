package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Account
import com.radixdlt.sargon.newAccountSampleMainnetAlice
import com.radixdlt.sargon.newAccountSampleMainnetBob
import com.radixdlt.sargon.newAccountSampleMainnetCarol
import com.radixdlt.sargon.newAccountSampleStokenetNadia
import com.radixdlt.sargon.newAccountSampleStokenetOlivia
import com.radixdlt.sargon.newAccountSampleStokenetPaige

@VisibleForTesting
object AccountMainnetSample: Sample<Account> {
    override fun invoke(): Account = alice
    override fun other(): Account = bob

    val alice: Account
        get() = newAccountSampleMainnetAlice()

    val bob: Account
        get() = newAccountSampleMainnetBob()

    val carol: Account
        get() = newAccountSampleMainnetCarol()

}

@VisibleForTesting
val Account.Companion.sampleMainnet: AccountMainnetSample
    get() = AccountMainnetSample

@VisibleForTesting
object AccountStokenetSample: Sample<Account> {
    override fun invoke(): Account = nadia
    override fun other(): Account = olivia

    val nadia: Account
        get() = newAccountSampleStokenetNadia()

    val olivia: Account
        get() = newAccountSampleStokenetOlivia()

    val paige: Account
        get() = newAccountSampleStokenetPaige()

}

@VisibleForTesting
val Account.Companion.sampleStokenet: AccountStokenetSample
    get() = AccountStokenetSample