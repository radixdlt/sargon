package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PersonaDataEntryEmailAddress
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressFromJsonString
import com.radixdlt.sargon.personaDataEntryEmailAddressToJsonString

@Throws(SargonException::class)
fun PersonaDataEntryEmailAddress.Companion.fromJson(
    jsonString: String
): PersonaDataEntryEmailAddress =
    newPersonaDataEntryEmailAddressFromJsonString(jsonString = jsonString)

fun PersonaDataEntryEmailAddress.toJson(): String =
    personaDataEntryEmailAddressToJsonString(personaDataEntryEmailAddress = this)