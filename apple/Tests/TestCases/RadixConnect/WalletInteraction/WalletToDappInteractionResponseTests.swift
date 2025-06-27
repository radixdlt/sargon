import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class WalletToDappInteractionResponseTests: Test<WalletToDappInteractionResponse> {
	func test_codable() throws {
		let json = try jsonData(
			file: "wallet_interactions_wallet_to_dapp",
			in: "models/interaction"
		)
		let sut = try JSONDecoder().decode([SUT].self, from: json)
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode([SUT].self, from: encoded), sut)
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
