import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AuthorizedDappTests: Test<AuthorizedDapp> {

	func test_network_ids_mainnet() {
		XCTAssertTrue(SUT.sampleValuesMainnet.allSatisfy({ $0.networkID == .mainnet }))
	}

	func test_network_ids_stokenet() {
		XCTAssertTrue(SUT.sampleValuesStokenet.allSatisfy({ $0.networkID == .stokenet }))
	}

	func test_id_is_dapp_definition() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.dAppDefinitionAddress)
		}
	}

	func test_codable() throws {
		let raw = """
				{
					  "networkID": 1,
					  "dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
					  "displayName": "Gumball Club",
					  "referencesToAuthorizedPersonas": [
						  {
							  "identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
							  "lastLogin": "2024-01-31T14:23:45.000Z",
							  "sharedAccounts": {
								  "request": {
									  "quantifier": "atLeast",
									  "quantity": 1
								  },
								  "ids": [
									  "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
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
		XCTAssertNoDifference(sut, SUT.sampleMainnetOther)

		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
