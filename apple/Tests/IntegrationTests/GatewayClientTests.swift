import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

// MARK: - GatewayClientTests
final class GatewayClientTests: TestCase {
	func test_xrd_balance_of_account() async throws {
		let gateway = GatewayClient(networkID: .mainnet)
		let xrdBalance = try await gateway.xrdBalanceOfAccountOrZero(address: .sample)
		XCTAssertGreaterThan(xrdBalance, 1)
	}
}

// MARK: - NetworkResponseTests
final class NetworkResponseTests: TestCase {
	typealias SUT = NetworkResponse
	func test_response_not_http_url() throws {
		XCTAssertThrowsError(
			try SUT(
				response: (Data(), URLResponse())
			)
		)
	}
}

// MARK: - NetworkAntennaTests
final class NetworkAntennaTests: TestCase {
	func test_url_session_executeNetworkRequest_bad_response() async throws {
		struct Fail: Swift.Error {}

		class MockURLProtocol: URLProtocol {
			override class func canInit(with request: URLRequest) -> Bool {
				true
			}

			override class func canonicalRequest(for request: URLRequest) -> URLRequest {
				request
			}

			override func startLoading() {
				self.client?.urlProtocol(self, didFailWithError: Fail())
			}

			override func stopLoading() {}
		}

		let failURLSession: URLSession = {
			let configuration = URLSessionConfiguration.ephemeral
			configuration.protocolClasses = [MockURLProtocol.self]
			return URLSession(configuration: configuration)
		}()

		let failGateway = GatewayClient(networkAntenna: failURLSession, networkId: .mainnet)
		do {
			_ = try await failGateway.xrdBalanceOfAccountOrZero(address: AccountAddress.sample)
			XCTFail("Expected to fail")
		} catch {
			// Good expected to fail!
		}
	}
}
