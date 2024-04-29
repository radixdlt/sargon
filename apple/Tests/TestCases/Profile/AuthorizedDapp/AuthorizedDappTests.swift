import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AuthorizedDappTests: Test<AuthorizedDapp> {
	func test_network_ids_mainnet() {
		XCTAssertTrue(SUT.sampleValuesMainnet.allSatisfy { $0.networkID == .mainnet })
	}

	func test_network_ids_stokenet() {
		XCTAssertTrue(SUT.sampleValuesStokenet.allSatisfy { $0.networkID == .stokenet })
	}

	func test_id_is_dapp_definition() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.id, sut.dAppDefinitionAddress)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_codable() throws {
		let raw = """
			{
				  "networkID": 1,
				  "dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				  "displayName": "Gumball Club",
				  "referencesToAuthorizedPersonas": [
					  {
						  "identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
						  "lastLogin": "2024-01-31T14:23:45.000Z",
						  "sharedAccounts": {
							  "request": {
								  "quantifier": "atLeast",
								  "quantity": 1
							  },
							  "ids": [
								  "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
							  ]
						  },
						  "sharedPersonaData": {
							  "name": "00000000-0000-0000-0000-000000000000",
							  "emailAddresses": {
								  "request": {
									  "quantifier": "exactly",
									  "quantity": 1
								  },
								  "ids": [
									  "00000000-0000-0000-0000-000000000002"
								  ]
							  },
							  "phoneNumbers": {
								  "request": {
									  "quantifier": "exactly",
									  "quantity": 1
								  },
								  "ids": [
									  "00000000-0000-0000-0000-000000000001"
								  ]
							  }
						  }
					  }
				  ]
			  }
		""".data(using: .utf8)!

		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sampleMainnetOther)

		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}

	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
}
