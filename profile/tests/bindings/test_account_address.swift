import radix_wallet_kit

func testAddress() throws {
    let bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
    let key = try Ed25519PublicKey.fromHex(hex: "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d")
    let address0 = AccountAddress(publicKey: PublicKey.ed25519(key: key), networkId: .mainnet)
    assert(address0.address() == bech32)
    let address1 = try AccountAddress.fromBech32(string: bech32)
    assert(address1.address() == bech32)

}

func test() throws {
    try testAddress()
}

try! test()