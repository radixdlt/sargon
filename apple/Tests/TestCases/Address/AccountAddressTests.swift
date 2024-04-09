final class AccountAddressTests: AddressTest<AccountAddress> {
    
    func testAddress() throws {
        
        let key = try Ed25519PublicKey(
            hex: "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d"
        )
        
        let address0 = AccountAddress(
            publicKey: PublicKey.ed25519(value: key), 
            networkID: .mainnet
        )
		
        XCTAssertEqual(
			address0.address, "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
		)
    }
	
	func test_into_fails_for_wrong_address_type() {
		XCTAssertThrowsError(try SUT.sample.asGeneral.asSpecific(type: IdentityAddress.self))
	}
	
	func test_short() {
		XCTAssertEqual(SUT.sample.shortFormat, "acco...nvjdwr")
	}
    
    func test_from_bech32_on_stokenet() throws {
        let address = try SUT(
            validatingAddress: "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk"
        )
        XCTAssertEqual(address.networkID, .stokenet)

        XCTAssertEqual(
            address,
            "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk" // ExpressibleByStringLiteral
        )
    }
	
	func test_is_legacy() {
		XCTAssertTrue(SUT("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease").isLegacy)

		// not legacy
		XCTAssertFalse(SUT.sampleStokenet.isLegacy)
		XCTAssertFalse(SUT.sampleStokenetOther.isLegacy)
	}
}
