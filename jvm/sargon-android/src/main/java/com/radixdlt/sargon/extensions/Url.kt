package com.radixdlt.sargon.extensions

import android.net.Uri
import android.util.Size
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.Url
import com.radixdlt.sargon.VectorImageType
import com.radixdlt.sargon.imageUrlUtilsIsVectorImage
import com.radixdlt.sargon.imageUrlUtilsMakeImageUrl
import okhttp3.HttpUrl.Companion.toHttpUrl
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

fun String.toUrl() = toHttpUrl()
fun String.toUrlOrNull() = toHttpUrlOrNull()

fun Uri.isVectorImage(imageType: VectorImageType): Boolean = imageUrlUtilsIsVectorImage(
    url = toString(),
    imageType = imageType
)

@Throws(CommonException::class)
fun Uri.intoImageUrl(
    imageServiceUrl: Url,
    size: Size
): Url = imageUrlUtilsMakeImageUrl(
    url = toString(),
    imageServiceUrl = imageServiceUrl.toString(),
    width = size.width.toUInt(),
    height = size.height.toUInt()
)