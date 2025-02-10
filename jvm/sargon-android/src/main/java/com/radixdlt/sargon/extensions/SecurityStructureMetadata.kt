package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SecurityStructureMetadata
import com.radixdlt.sargon.extensions.securityStructureIsMain

val SecurityStructureMetadata.isMain
    get() = securityStructureIsMain(securityStructureMetadata = this)