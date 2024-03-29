package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AccountPath
import com.radixdlt.sargon.Bip44LikePath
import com.radixdlt.sargon.IdentityPath
import com.radixdlt.sargon.newAccountPathSample
import com.radixdlt.sargon.newAccountPathSampleOther
import com.radixdlt.sargon.newBip44LikePathSample
import com.radixdlt.sargon.newBip44LikePathSampleOther
import com.radixdlt.sargon.newIdentityPathSample
import com.radixdlt.sargon.newIdentityPathSampleOther

@VisibleForTesting
val AccountPath.Companion.sample: Sample<AccountPath>
    get() = object : Sample<AccountPath> {
        override fun invoke(): AccountPath = newAccountPathSample()

        override fun other(): AccountPath = newAccountPathSampleOther()
    }

@VisibleForTesting
val IdentityPath.Companion.sample: Sample<IdentityPath>
    get() = object : Sample<IdentityPath> {
        override fun invoke(): IdentityPath = newIdentityPathSample()

        override fun other(): IdentityPath = newIdentityPathSampleOther()
    }

@VisibleForTesting
val Bip44LikePath.Companion.sample: Sample<Bip44LikePath>
    get() = object : Sample<Bip44LikePath> {
        override fun invoke(): Bip44LikePath = newBip44LikePathSample()

        override fun other(): Bip44LikePath = newBip44LikePathSampleOther()
    }

class AccountPathPreviewParameterProvider : PreviewParameterProvider<AccountPath> {
    override val values: Sequence<AccountPath>
        get() = AccountPath.sample.all.asSequence()

}

class IdentityPathPreviewParameterProvider : PreviewParameterProvider<IdentityPath> {
    override val values: Sequence<IdentityPath>
        get() = IdentityPath.sample.all.asSequence()

}

class Bip44LikePathPreviewParameterProvider : PreviewParameterProvider<Bip44LikePath> {
    override val values: Sequence<Bip44LikePath>
        get() = Bip44LikePath.sample.all.asSequence()

}