import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceCryptoParametersTests: Test<FactorSourceCryptoParameters> {
	func test_supports_olympia() {
		let f: (SUT) -> Bool = \.supportsOlympia

		XCTAssertTrue(f(.olympia))
		XCTAssertTrue(f(.babylonOlympiaCompatible))
		XCTAssertFalse(f(.babylon))
	}

	func test_supports_babylon() {
		let f: (SUT) -> Bool = \.supportsBabylon

		XCTAssertFalse(f(.olympia))
		XCTAssertTrue(f(.babylonOlympiaCompatible))
		XCTAssertTrue(f(.babylon))
	}
}
