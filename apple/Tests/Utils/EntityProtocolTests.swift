class EntityTest<SUT_: EntityProtocol>: Test<SUT_> {
	
	func test_network_id_of_mainnet_entities() {
		SUT.sampleValuesMainnet.forEach {
			XCTAssertNoDifference($0.networkID, .mainnet)
		}
	}
	
	func test_network_id_of_stokenet_entities() {
		SUT.sampleValuesStokenet.forEach {
			XCTAssertNoDifference($0.networkID, .stokenet)
		}
	}
	
	func test_id_is_address() {
		SUT.allCases.forEach {
			XCTAssertNoDifference($0.id, $0.address)
		}
	}

	func test_controlled_by_ed25519_factor() {
		SUT.allCases.forEach {
			switch $0.securityState {
			case .unsecured(let unsecuredEntityControl):
				switch 	unsecuredEntityControl.transactionSigning.publicKey.publicKey {
				case .ed25519: break // good
				case .secp256k1: XCTFail("Wrong key kind")
				}
			}
		}
	}

	
	func test_all_address_different() {
		XCTAssertGreaterThanOrEqual(Set(SUT.allCases).count, 6)
	}
}
