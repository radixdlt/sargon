package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.HostOs
import com.radixdlt.sargon.hostOsGetName
import com.radixdlt.sargon.hostOsGetVendor
import com.radixdlt.sargon.hostOsGetVersion
import com.radixdlt.sargon.newHostOsAndroid
import com.radixdlt.sargon.newHostOsOther

fun HostOs.Companion.android(
    vendor: String,
    version: String
): HostOs = newHostOsAndroid(vendor = vendor, version = version)

fun HostOs.Companion.other(
    name: String,
    vendor: String,
    version: String
): HostOs = newHostOsOther(name = name, vendor = vendor, version = version)

val HostOs.name: String
    get() = hostOsGetName(hostOs = this)

val HostOs.vendor: String
    get() = hostOsGetVendor(hostOs = this)

val HostOs.version: String
    get() = hostOsGetVersion(hostOs = this)