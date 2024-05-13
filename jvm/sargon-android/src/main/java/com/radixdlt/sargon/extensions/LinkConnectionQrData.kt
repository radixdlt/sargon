package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.LinkConnectionQrData
import com.radixdlt.sargon.linkConnectionQRDataToJsonBytes
import com.radixdlt.sargon.newLinkConnectionQRDataFromJsonBytes

@Throws(SargonException::class)
fun LinkConnectionQrData.Companion.fromJson(json: String) =
    newLinkConnectionQRDataFromJsonBytes(jsonBytes = bagOfBytes(json))

fun LinkConnectionQrData.toJson(): String =
    linkConnectionQRDataToJsonBytes(this).string