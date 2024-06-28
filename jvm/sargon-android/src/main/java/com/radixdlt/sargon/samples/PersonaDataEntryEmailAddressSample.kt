package com.radixdlt.sargon.samples

import com.radixdlt.sargon.EmailAddress
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressSample
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressSampleOther

@UsesSampleValues
val EmailAddress.Companion.sample: Sample<EmailAddress>
    get() = object : Sample<EmailAddress> {
        override fun invoke(): EmailAddress = newPersonaDataEntryEmailAddressSample()

        override fun other(): EmailAddress = newPersonaDataEntryEmailAddressSampleOther()

    }