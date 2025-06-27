import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDTests: FactorSourceIDTest<FactorSourceID> {
	func test_as_general_is_self() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral, sut)
		}
	}

	func test_spot_check() {
		let input = SpotCheckInput.software(mnemonicWithPassphrase: .sample)
		XCTAssertTrue(SUT.sample.spotCheck(input: input))
	}
}
