package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.ManifestEncounteredAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.manifestEncounteredAddressFormatted
import com.radixdlt.sargon.manifestEncounteredAddressNetworkId
import com.radixdlt.sargon.manifestEncounteredAddressToString
import com.radixdlt.sargon.newManifestEncounteredAddressFromBech32

@Throws(SargonException::class)
fun ManifestEncounteredAddress.Companion.init(validating: String) =
    newManifestEncounteredAddressFromBech32(string = validating)

val ManifestEncounteredAddress.string: String
    get() = manifestEncounteredAddressToString(address = this)

val ManifestEncounteredAddress.networkId: NetworkId
    get() = manifestEncounteredAddressNetworkId(address = this)

fun ManifestEncounteredAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = manifestEncounteredAddressFormatted(address = this, format = format)