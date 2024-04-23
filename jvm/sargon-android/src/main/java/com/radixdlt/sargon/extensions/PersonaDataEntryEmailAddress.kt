package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PersonaDataEntryEmailAddress
import com.radixdlt.sargon.newPersonaDataEntryEmailAddressFromJsonString
import com.radixdlt.sargon.personaDataEntryEmailAddressToJsonString

@Throws(SargonException::class)
fun PersonaDataEntryEmailAddress.Companion.deserializeFromJsonString(
    jsonString: String
): PersonaDataEntryEmailAddress =
    newPersonaDataEntryEmailAddressFromJsonString(jsonString = jsonString)

fun PersonaDataEntryEmailAddress.serializedJsonString(): String =
    personaDataEntryEmailAddressToJsonString(personaDataEntryEmailAddress = this)