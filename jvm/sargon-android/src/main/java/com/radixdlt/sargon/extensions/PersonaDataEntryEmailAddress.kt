package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.EmailAddress
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressFromJsonString
import com.radixdlt.sargon.personaDataEntryEmailAddressToJsonString

typealias PersonaDataEntryEmailAddress = EmailAddress

@Throws(SargonException::class)
fun EmailAddress.Companion.fromJson(
    jsonString: String
): EmailAddress =
    newPersonaDataEntryEmailAddressFromJsonString(jsonString = jsonString)

fun EmailAddress.toJson(): String =
    personaDataEntryEmailAddressToJsonString(personaDataEntryEmailAddress = this)