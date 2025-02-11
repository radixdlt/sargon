package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SecurityStructureMetadata
import com.radixdlt.sargon.securityStructureMetadataIsMain

val SecurityStructureMetadata.isMain
    get() = securityStructureMetadataIsMain(securityStructureMetadata = this)