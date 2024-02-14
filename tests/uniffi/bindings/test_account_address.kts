import sargon.*

fun testAddress() {
    val bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
	val key = newEd25519PublicKeyFromHex(
		hex = "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d"
    )

	val address0 = newAccountAddressFrom(
        publicKey = PublicKey.Ed25519(value = key),
        networkId = NetworkId.MAINNET
    )
    assert(address0.address == bech32)

    val address1 = newAccountAddress(bech32 = bech32)
    assert(address1.address == bech32)
    assert(accountAddressToShort(address = address1) == "acco...m2kzdm")
    assert(address1.networkId == NetworkId.MAINNET)
}

fun test() {
	testAddress()
}

test()
