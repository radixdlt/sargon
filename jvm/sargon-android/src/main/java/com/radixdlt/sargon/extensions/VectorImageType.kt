package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.VectorImageType
import com.radixdlt.sargon.vectorImageTypeDataUrlType
import com.radixdlt.sargon.vectorImageTypeUrlExtension

val VectorImageType.urlExtension: String
    get() = vectorImageTypeUrlExtension(imageType = this)

val VectorImageType.dataUrlType: String
    get() = vectorImageTypeDataUrlType(imageType = this)
