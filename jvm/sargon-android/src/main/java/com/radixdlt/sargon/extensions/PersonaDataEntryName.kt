package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PersonaDataEntryName
import com.radixdlt.sargon.newPersonaDataEntryNameFromJsonBytes
import com.radixdlt.sargon.personaDataEntryNameToJsonBytes

@Throws(SargonException::class)
fun PersonaDataEntryName.Companion.deserializeFromJsonString(
    jsonString: String
): PersonaDataEntryName = newPersonaDataEntryNameFromJsonBytes(jsonBytes = bagOfBytes(jsonString))

fun PersonaDataEntryName.serializedJsonString(): String =
    personaDataEntryNameToJsonBytes(personaDataEntryName = this).string