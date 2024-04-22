package com.radixdlt.sargon.samples

import com.radixdlt.sargon.PersonaDataEntryName
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPersonaDataEntryNameSample
import com.radixdlt.sargon.newPersonaDataEntryNameSampleOther

@UsesSampleValues
val PersonaDataEntryName.Companion.sample: Sample<PersonaDataEntryName>
    get() = object : Sample<PersonaDataEntryName> {
        override fun invoke(): PersonaDataEntryName = newPersonaDataEntryNameSample()

        override fun other(): PersonaDataEntryName = newPersonaDataEntryNameSampleOther()

    }