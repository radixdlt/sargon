package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PersonaDataEntryName
import com.radixdlt.sargon.newPersonaDataEntryNameFromJsonBytes
import com.radixdlt.sargon.personaDataEntryNameToJsonBytes

@Throws(SargonException::class)
fun PersonaDataEntryName.Companion.fromJson(
    jsonString: String
): PersonaDataEntryName = newPersonaDataEntryNameFromJsonBytes(jsonBytes = bagOfBytes(jsonString))

fun PersonaDataEntryName.toJson(): String =
    personaDataEntryNameToJsonBytes(personaDataEntryName = this).string