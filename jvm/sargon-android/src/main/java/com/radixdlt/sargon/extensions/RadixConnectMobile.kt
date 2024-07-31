package com.radixdlt.sargon.extensions

import android.content.Context
import com.radixdlt.sargon.RadixConnectMobile
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.os.driver.AndroidNetworkingDriver
import com.radixdlt.sargon.os.radixconnect.RadixConnectSessionStorage
import okhttp3.OkHttpClient

@KoverIgnore
fun RadixConnectMobile.Companion.init(
    context: Context,
    okHttpClient: OkHttpClient
) = RadixConnectMobile(
    networkingDriver = AndroidNetworkingDriver(client = okHttpClient),
    sessionStorage = RadixConnectSessionStorage(context = context)
)