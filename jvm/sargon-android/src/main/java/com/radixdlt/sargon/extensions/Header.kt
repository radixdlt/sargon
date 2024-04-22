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
fun Header.Companion.deserializeFromJsonBytes(jsonBytes: BagOfBytes) =
    newHeaderFromJsonBytes(jsonBytes = jsonBytes)

@Throws(SargonException::class)
fun Header.Companion.deserializeFromJsonString(jsonString: String) =
    deserializeFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun Header.serializedJsonBytes(): BagOfBytes = headerToJsonBytes(header = this)

fun Header.serializedJsonString(): String = serializedJsonBytes().string