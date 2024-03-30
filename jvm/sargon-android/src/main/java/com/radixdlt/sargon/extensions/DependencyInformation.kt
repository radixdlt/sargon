package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DependencyInformation
import com.radixdlt.sargon.dependencyInformationToString

val DependencyInformation.string: String
    get() = dependencyInformationToString(info = this)