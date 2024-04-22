package com.radixdlt.sargon.samples

import com.radixdlt.sargon.PersonaDataEntryPhoneNumber
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPersonaDataEntryPhoneNumberSample
import com.radixdlt.sargon.newPersonaDataEntryPhoneNumberSampleOther

@UsesSampleValues
val PersonaDataEntryPhoneNumber.Companion.sample: Sample<PersonaDataEntryPhoneNumber>
    get() = object : Sample<PersonaDataEntryPhoneNumber> {
        override fun invoke(): PersonaDataEntryPhoneNumber = newPersonaDataEntryPhoneNumberSample()

        override fun other(): PersonaDataEntryPhoneNumber =
            newPersonaDataEntryPhoneNumberSampleOther()

    }