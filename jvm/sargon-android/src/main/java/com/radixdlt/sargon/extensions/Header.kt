package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceInfo
import com.radixdlt.sargon.Header
import com.radixdlt.sargon.headerToJsonBytes
import com.radixdlt.sargon.newHeaderFromJsonBytes
import com.radixdlt.sargon.newHeaderWithCreatingDevice

fun Header.Companion.init(creatingDevice: DeviceInfo): Header =
    newHeaderWithCreatingDevice(creatingDevice = creatingDevice)

@Throws(SargonException::class)
fun Header.Companion.fromJson(jsonString: String) =
    newHeaderFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun Header.toJson(): String = headerToJsonBytes(header = this).string