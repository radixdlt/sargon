package com.radixdlt.sargon.samples

import com.radixdlt.sargon.DeviceInfo
import com.radixdlt.sargon.Timestamp
import com.radixdlt.sargon.annotation.UsesSampleValues
import java.util.UUID

@UsesSampleValues
val DeviceInfo.Companion.sample: Sample<DeviceInfo>
    get() = object : Sample<DeviceInfo> {
        override fun invoke(): DeviceInfo = DeviceInfo(
            id = UUID.fromString("6b3b43cd-135f-418b-9673-aef82cd016b5"),
            date = Timestamp.parse("2024-05-28T15:01:49.067Z"),
            description = "Michael's Google Pixel 8 XL",
        )

        override fun other(): DeviceInfo = DeviceInfo(
            id = UUID.fromString("2b8d3513-7e77-4a5a-b486-68877042c57e"),
            date = Timestamp.parse("2024-05-28T15:02:32.324Z"),
            description = "Michael's Google Pixel 5",
        )

    }