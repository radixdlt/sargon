package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileFileContents
import com.radixdlt.sargon.checkIfEncryptedProfileJsonContainsLegacyP2pLinks
import com.radixdlt.sargon.checkIfProfileJsonContainsLegacyP2pLinks
import com.radixdlt.sargon.newProfile
import com.radixdlt.sargon.newProfileFromEncryptionBytes
import com.radixdlt.sargon.newProfileFromJsonString
import com.radixdlt.sargon.profileAnalyzeContentsOfFile
import com.radixdlt.sargon.profileEncryptWithPassword
import com.radixdlt.sargon.profileToJsonString

fun Profile.Companion.init(
    deviceFactorSource: FactorSource.Device,
    hostId: HostId,
    hostInfo: HostInfo
) = newProfile(
    deviceFactorSource = deviceFactorSource.value,
    hostId = hostId,
    hostInfo = hostInfo
)

fun Profile.Companion.analyzeContentsOfFile(contents: String): ProfileFileContents =
    profileAnalyzeContentsOfFile(contents = contents)

@Throws(SargonException::class)
fun Profile.Companion.fromJson(jsonString: String) =
    newProfileFromJsonString(jsonStr = jsonString)

@Throws(SargonException::class)
fun Profile.Companion.fromEncryptedJson(
    jsonString: String,
    decryptionPassword: String
) = newProfileFromEncryptionBytes(
    jsonString = jsonString,
    decryptionPassword = decryptionPassword
)

fun Profile.toJson(prettyPrinted: Boolean = true) = profileToJsonString(profile = this, prettyPrinted = prettyPrinted)
fun Profile.toEncryptedJson(encryptionPassword: String) =
    profileEncryptWithPassword(profile = this, encryptionPassword = encryptionPassword)

fun Profile.Companion.checkIfProfileJsonContainsLegacyP2PLinks(jsonString: String) =
    checkIfProfileJsonContainsLegacyP2pLinks(jsonStr = jsonString)

fun Profile.Companion.checkIfEncryptedProfileJsonContainsLegacyP2PLinks(jsonString: String, password: String) =
    checkIfEncryptedProfileJsonContainsLegacyP2pLinks(jsonStr = jsonString, password)