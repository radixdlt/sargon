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
				url: URL(
					string: "https://radixdlt.com"
				)!,
				method: .head,
				headers: [:],
				body: .init()
			)
		)
		XCTAssertEqual(response.statusCode, 200)
	}
}
