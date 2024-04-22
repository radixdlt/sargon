package com.radixdlt.sargon.samples

import com.radixdlt.sargon.PersonaDataEntryEmailAddress
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressSample
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressSampleOther

@UsesSampleValues
val PersonaDataEntryEmailAddress.Companion.sample: Sample<PersonaDataEntryEmailAddress>
    get() = object : Sample<PersonaDataEntryEmailAddress> {
        override fun invoke(): PersonaDataEntryEmailAddress = newPersonaDataEntryEmailAddressSample()

        override fun other(): PersonaDataEntryEmailAddress = newPersonaDataEntryEmailAddressSampleOther()

    }