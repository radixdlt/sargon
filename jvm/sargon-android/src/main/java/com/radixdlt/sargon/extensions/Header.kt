package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.DeviceInfo
import com.radixdlt.sargon.Header
import com.radixdlt.sargon.headerToJsonBytes
import com.radixdlt.sargon.newHeaderFromJsonBytes
import com.radixdlt.sargon.newHeaderWithCreatingDevice

fun Header.Companion.init(creatingDevice: DeviceInfo): Header =
    newHeaderWithCreatingDevice(creatingDevice = creatingDevice)

@Throws(SargonException::class)
fun Header.Companion.deserializeFromBytes(jsonBytes: BagOfBytes) =
    newHeaderFromJsonBytes(jsonBytes = jsonBytes)

@Throws(SargonException::class)
fun Header.Companion.deserializeFromString(jsonString: String) =
    deserializeFromBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun Header.serializedBytes(): BagOfBytes = headerToJsonBytes(header = this)

fun Header.serializedString(): String = serializedBytes().string