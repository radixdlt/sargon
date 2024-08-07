package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PersonaDataEntryPhoneNumber
import com.radixdlt.sargon.newPersonaDataEntryPhoneNumberFromJsonString
import com.radixdlt.sargon.personaDataEntryPhoneNumberToJsonString

@Throws(SargonException::class)
fun PersonaDataEntryPhoneNumber.Companion.fromJson(
    jsonString: String
): PersonaDataEntryPhoneNumber =
    newPersonaDataEntryPhoneNumberFromJsonString(jsonString = jsonString)

fun PersonaDataEntryPhoneNumber.toJson(): String =
    personaDataEntryPhoneNumberToJsonString(personaDataEntryPhoneNumber = this)