package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ReferencesToAuthorizedPersonas
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasSample
import com.radixdlt.sargon.newReferencesToAuthorizedPersonasSampleOther

@UsesSampleValues
val ReferencesToAuthorizedPersonas.Companion.sample: Sample<ReferencesToAuthorizedPersonas>
    get() = object : Sample<ReferencesToAuthorizedPersonas> {
        override fun invoke(): ReferencesToAuthorizedPersonas = newReferencesToAuthorizedPersonasSample()

        override fun other(): ReferencesToAuthorizedPersonas =
            newReferencesToAuthorizedPersonasSampleOther()

    }