package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceInfoDescription
import com.radixdlt.sargon.deviceInfoDescriptionToString

val DeviceInfoDescription.string: String
    get() = deviceInfoDescriptionToString(deviceInfoDescription = this)