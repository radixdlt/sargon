package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AuthorizedPersonaSimple
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAuthorizedPersonaSimpleSampleMainnet
import com.radixdlt.sargon.newAuthorizedPersonaSimpleSampleMainnetOther
import com.radixdlt.sargon.newAuthorizedPersonaSimpleSampleStokenet
import com.radixdlt.sargon.newAuthorizedPersonaSimpleSampleStokenetOther

@UsesSampleValues
val AuthorizedPersonaSimple.Companion.sampleMainnet: Sample<AuthorizedPersonaSimple>
    get() = object : Sample<AuthorizedPersonaSimple> {
        override fun invoke(): AuthorizedPersonaSimple = newAuthorizedPersonaSimpleSampleMainnet()

        override fun other(): AuthorizedPersonaSimple =
            newAuthorizedPersonaSimpleSampleMainnetOther()

    }

@UsesSampleValues
val AuthorizedPersonaSimple.Companion.sampleStokenet: Sample<AuthorizedPersonaSimple>
    get() = object : Sample<AuthorizedPersonaSimple> {
        override fun invoke(): AuthorizedPersonaSimple = newAuthorizedPersonaSimpleSampleStokenet()

        override fun other(): AuthorizedPersonaSimple =
            newAuthorizedPersonaSimpleSampleStokenetOther()

    }