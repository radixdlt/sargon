import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BIP39WordTest: Test<BIP39Word> {
	func test_id_is_index() {
		XCTAssertEqual(SUT.sample.id, U11(inner: 0))
		XCTAssertEqual(SUT.sampleOther.id, U11(inner: 2047))
	}
}
