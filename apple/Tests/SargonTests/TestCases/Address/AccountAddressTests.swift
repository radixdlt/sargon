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
	
	func test_short() {
		XCTAssertEqual(SUT.sample.shortFormat, "acco...please")
	}
}
