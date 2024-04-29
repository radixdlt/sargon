import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AssetExceptionTests: Test<AssetException> {
	func test_id_is_address() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.id, sut.address)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
