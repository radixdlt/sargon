package com.radixdlt.sargon.samples

import com.radixdlt.sargon.PersonaData
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPersonaDataSample
import com.radixdlt.sargon.newPersonaDataSampleOther

@UsesSampleValues
val PersonaData.Companion.sample: Sample<PersonaData>
    get() = object : Sample<PersonaData> {
        override fun invoke(): PersonaData = newPersonaDataSample()

        override fun other(): PersonaData = newPersonaDataSampleOther()

    }