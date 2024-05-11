package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileFileContents
import com.radixdlt.sargon.checkIfProfileJsonContainsLegacyP2pLinks
import com.radixdlt.sargon.newProfile
import com.radixdlt.sargon.newProfileFromEncryptionBytes
import com.radixdlt.sargon.newProfileFromJsonBytes
import com.radixdlt.sargon.profileAnalyzeContentsOfFile
import com.radixdlt.sargon.profileEncryptWithPassword
import com.radixdlt.sargon.profileToJsonBytes

fun Profile.Companion.init(
    deviceFactorSource: FactorSource.Device,
    creatingDeviceName: String
) = newProfile(
    deviceFactorSource = deviceFactorSource.value,
    creatingDeviceName = creatingDeviceName
)

fun Profile.Companion.analyzeContentsOfFile(contents: String): ProfileFileContents =
    profileAnalyzeContentsOfFile(bytes = bagOfBytes(fromString = contents))

@Throws(SargonException::class)
fun Profile.Companion.fromJson(jsonString: String) =
    newProfileFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

@Throws(SargonException::class)
fun Profile.Companion.fromEncryptedJson(
    jsonString: String,
    decryptionPassword: String
) = newProfileFromEncryptionBytes(
    json = bagOfBytes(fromString = jsonString),
    decryptionPassword = decryptionPassword
)

fun Profile.toJson() = profileToJsonBytes(profile = this).string
fun Profile.toEncryptedJson(encryptionPassword: String) =
    profileEncryptWithPassword(profile = this, encryptionPassword = encryptionPassword).string

fun Profile.Companion.checkIfProfileJsonContainsLegacyP2PLinks(jsonString: String) =
    checkIfProfileJsonContainsLegacyP2pLinks(json = bagOfBytes(fromString = jsonString))