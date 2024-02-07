import radix.wallet.kit.*

fun test() {
	val bech32 = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
	val address = newResourceAddress(bech32 = bech32)
	assert(address.address == bech32)
	assert(address.networkId == NetworkId.MAINNET)
}

test()
