package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.buildInformation

object Sargon {

    val buildInformation: SargonBuildInformation
        get() = buildInformation()

}