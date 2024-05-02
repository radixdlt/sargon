package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileFileContents
import com.radixdlt.sargon.RefBytes
import com.radixdlt.sargon.RefProfile
import com.radixdlt.sargon.newProfile
import com.radixdlt.sargon.newProfileFromEncryptionBytesFastByRef
import com.radixdlt.sargon.newProfileFromJsonBytesFastByRef
import com.radixdlt.sargon.profileAnalyzeContentsOfFileFastByRef
import com.radixdlt.sargon.profileEncryptWithPasswordFastByRef
import com.radixdlt.sargon.profileToJsonBytesFastByRef

fun Profile.Companion.init(
    deviceFactorSource: FactorSource.Device,
    creatingDeviceName: String
) = newProfile(
    deviceFactorSource = deviceFactorSource.value,
    creatingDeviceName = creatingDeviceName
)

fun Profile.Companion.analyzeContentsOfFile(contents: String): ProfileFileContents =
    profileAnalyzeContentsOfFileFastByRef(reference = RefBytes(inner = bagOfBytes(fromString = contents)))

@Throws(SargonException::class)
fun Profile.Companion.fromJson(jsonString: String) =
    newProfileFromJsonBytesFastByRef(reference = RefBytes(inner = bagOfBytes(fromString = jsonString))).take()

@Throws(SargonException::class)
fun Profile.Companion.fromEncryptedJson(
    jsonString: String,
    decryptionPassword: String
) = newProfileFromEncryptionBytesFastByRef(
    reference = RefBytes(inner = bagOfBytes(fromString = jsonString)),
    decryptionPassword = decryptionPassword
).take()

fun Profile.toJson() = profileToJsonBytesFastByRef(reference = RefProfile(inner = this)).take().string
fun Profile.toEncryptedJson(encryptionPassword: String) =
    profileEncryptWithPasswordFastByRef(reference = RefProfile(inner = this), encryptionPassword = encryptionPassword).take().string
