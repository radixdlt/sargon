package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.buildInformation
import com.radixdlt.sargon.annotation.KoverIgnore

object Sargon {

    val buildInformation: SargonBuildInformation
        @KoverIgnore
        get() = buildInformation()

}