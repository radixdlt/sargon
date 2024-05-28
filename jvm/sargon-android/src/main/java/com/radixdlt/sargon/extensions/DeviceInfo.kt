package com.radixdlt.sargon.extensions

import android.content.Context
import android.os.Build
import android.provider.Settings
import com.radixdlt.sargon.DeviceInfo
import com.radixdlt.sargon.Timestamp
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.deviceInfoToJsonBytes
import com.radixdlt.sargon.newDeviceInfoFromJsonBytes
import java.util.Locale
import java.util.UUID

@KoverIgnore
fun DeviceInfo.Companion.generate(context: Context) = DeviceInfo(
    id = UUID.randomUUID(),
    date = Timestamp.now(),
    description = DeviceDescription(context).description
)

fun DeviceInfo.Companion.fromJson(json: String) = newDeviceInfoFromJsonBytes(
    jsonBytes = bagOfBytes(fromString = json)
)

fun DeviceInfo.toJson() = deviceInfoToJsonBytes(deviceInfo = this).string

@KoverIgnore
class DeviceDescription private constructor(
    val name: String,
    val manufacturer: String,
    val model: String
) {

    constructor(context: Context) : this(
        name = Settings.Global.getString(
            context.contentResolver,
            Settings.Global.DEVICE_NAME
        ).orEmpty(),
        manufacturer = Build.MANUFACTURER.replaceFirstChar {
            if (it.isLowerCase()) it.titlecase(Locale.getDefault()) else it.toString()
        },
        model = Build.MODEL.replaceFirstChar {
            if (it.isLowerCase()) it.titlecase(Locale.getDefault()) else it.toString()
        }
    )

    private val commercialName: String
        get() = if (model.startsWith(manufacturer)) {
            model
        } else {
            "$manufacturer $model"
        }

    val description: String
        get() = if (name.isBlank()) {
            commercialName
        } else {
            "$name $commercialName"
        }

}