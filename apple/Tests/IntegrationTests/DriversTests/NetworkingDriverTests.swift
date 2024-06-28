import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class NetworkingDriverTests: DriverTest<URLSession> {
	
	func test() async throws {
		let sut = SUT.shared as NetworkingDriver
		let response = try await sut.executeNetworkRequest(
			request: .init(
				validating: "https://radixdlt.com",
				method: .head
			)
		)
		XCTAssertEqual(response.statusCode, 200)
	}
	
	func test_bad_url() {
		XCTAssertThrowsError(try URL(validating: ""))
	}
}
