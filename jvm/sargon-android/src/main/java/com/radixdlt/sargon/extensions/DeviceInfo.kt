package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceInfo
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.newDeviceInfoFromHostInfo

fun DeviceInfo.Companion.from(
    hostId: HostId,
    hostInfo: HostInfo
): DeviceInfo = newDeviceInfoFromHostInfo(hostId = hostId, hostInfo = hostInfo)