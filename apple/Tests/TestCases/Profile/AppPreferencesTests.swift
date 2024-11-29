import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AppPreferencesTests: Test<AppPreferences> {
	func test_default_guarantee_is_99() {
		XCTAssertEqual(SUT.default.transaction.defaultDepositGuarantee, 0.99)
	}

	func test_has_gateway_valid() throws {
		XCTAssertTrue(try SUT.default.hasGateway(with: .init(urlPath: "https://mainnet.radixdlt.com/")))
		XCTAssertTrue(try SUT.default.hasGateway(with: .init(urlPath: "https://mainnet.radixdlt.com")))
		XCTAssertFalse(try SUT.default.hasGateway(with: .init(urlPath: "https://radixdlt.com/")))
	}

	func test_has_gateway_invalid() {
		let urlPath = "invalid input"
		XCTAssertThrowsError(try SUT.default.hasGateway(with: .init(urlPath: urlPath))) { error in
			guard let commonError = error as? SargonUniFFI.CommonError else {
				return XCTFail("Expected CommonError")
			}
			XCTAssertEqual(commonError, .InvalidUrl(badValue: urlPath))
		}
	}
}
