package com.radixdlt.sargon.os.driver

import android.content.Context
import android.os.Build
import android.provider.Settings
import com.radixdlt.sargon.HostInfoDriver
import com.radixdlt.sargon.HostOs
import com.radixdlt.sargon.extensions.android
import java.util.Locale

class AndroidHostInfoDriver(
    private val context: Context
) : HostInfoDriver {

    override suspend fun hostOs(): HostOs = HostOs.android(
        vendor = getVendor(),
        version = getAndroidVersion()
    )

    override suspend fun hostDeviceName(): String = getDeviceName(context)

    override suspend fun hostAppVersion(): String = runCatching {
        context.applicationContext.packageManager.getPackageInfo(
            context.applicationContext.packageName,
            0
        ).versionName.takeUnless {
            // versionName may return "null" as a string
            it == "null"
        }
    }.getOrNull().orEmpty()

    override suspend fun hostDeviceModel(): String = getDeviceModel()

    private fun getDeviceName(context: Context) = Settings.Global.getString(
        context.applicationContext.contentResolver,
        Settings.Global.DEVICE_NAME
    ).orEmpty()

    private fun getVendor() = Build.MANUFACTURER.replaceFirstChar {
        if (it.isLowerCase()) it.titlecase(Locale.getDefault()) else it.toString()
    }

    private fun getDeviceModel() = Build.MODEL.replaceFirstChar {
        if (it.isLowerCase()) it.titlecase(Locale.getDefault()) else it.toString()
    }

    private fun getAndroidVersion(): String =
        "${Build.VERSION.RELEASE} (API ${Build.VERSION.SDK_INT})"
}