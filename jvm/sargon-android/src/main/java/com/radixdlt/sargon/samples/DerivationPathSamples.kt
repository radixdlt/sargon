package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AccountPath
import com.radixdlt.sargon.Bip44LikePath
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.IdentityPath
import com.radixdlt.sargon.newAccountPathSample
import com.radixdlt.sargon.newAccountPathSampleOther
import com.radixdlt.sargon.newBip44LikePathSample
import com.radixdlt.sargon.newBip44LikePathSampleOther
import com.radixdlt.sargon.newDerivationPathSample
import com.radixdlt.sargon.newDerivationPathSampleOther
import com.radixdlt.sargon.newIdentityPathSample
import com.radixdlt.sargon.newIdentityPathSampleOther

@UsesSampleValues
val DerivationPath.Companion.sample: Sample<DerivationPath>
    get() = object : Sample<DerivationPath> {
        override fun invoke(): DerivationPath = newDerivationPathSample()

        override fun other(): DerivationPath = newDerivationPathSampleOther()
    }

@UsesSampleValues
val AccountPath.Companion.sample: Sample<AccountPath>
    get() = object : Sample<AccountPath> {
        override fun invoke(): AccountPath = newAccountPathSample()

        override fun other(): AccountPath = newAccountPathSampleOther()
    }

@UsesSampleValues
val IdentityPath.Companion.sample: Sample<IdentityPath>
    get() = object : Sample<IdentityPath> {
        override fun invoke(): IdentityPath = newIdentityPathSample()

        override fun other(): IdentityPath = newIdentityPathSampleOther()
    }

@UsesSampleValues
val Bip44LikePath.Companion.sample: Sample<Bip44LikePath>
    get() = object : Sample<Bip44LikePath> {
        override fun invoke(): Bip44LikePath = newBip44LikePathSample()

        override fun other(): Bip44LikePath = newBip44LikePathSampleOther()
    }