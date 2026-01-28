import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class P2PLinkTests: Test<MfaFactorInstance> {
	func test_codable() throws {
		let raw = """
		{
		    "factorInstance": {
		        "badge": {
		            "virtualSource": {
		                "hierarchicalDeterministicPublicKey": {
		                    "publicKey": {
		                        "curve": "curve25519",
		                        "compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
		                    },
		                    "derivationPath": {
		                        "scheme": "cap26",
		                        "path": "m/44H/1022H/1H/525H/1460H/0H"
		                    }
		                },
		                "discriminator": "hierarchicalDeterministicPublicKey"
		            },
		            "discriminator": "virtualSource"
		        },
		        "factorSourceID": {
		            "fromHash": {
		                "kind": "device",
		                "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
		            },
		            "discriminator": "fromHash"
		        }
		    }
		}
		""".data(using: .utf8)!

		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)

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
