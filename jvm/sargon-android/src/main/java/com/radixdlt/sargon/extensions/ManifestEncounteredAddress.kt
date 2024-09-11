package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.ManifestEncounteredComponentAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.manifestEncounteredComponentAddressFormatted
import com.radixdlt.sargon.manifestEncounteredComponentAddressNetworkId
import com.radixdlt.sargon.manifestEncounteredComponentAddressToString
import com.radixdlt.sargon.newManifestEncounteredComponentAddressFromBech32

@Throws(SargonException::class)
fun ManifestEncounteredComponentAddress.Companion.init(validating: String) =
    newManifestEncounteredComponentAddressFromBech32(string = validating)

val ManifestEncounteredComponentAddress.string: String
    get() = manifestEncounteredComponentAddressToString(address = this)

val ManifestEncounteredComponentAddress.networkId: NetworkId
    get() = manifestEncounteredComponentAddressNetworkId(address = this)

fun ManifestEncounteredComponentAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = manifestEncounteredComponentAddressFormatted(address = this, format = format)

fun ManifestEncounteredComponentAddress.asGeneral() = when (this) {
    is ManifestEncounteredComponentAddress.Component -> Address.Component(v1)
    is ManifestEncounteredComponentAddress.Locker -> Address.Locker(v1)
}