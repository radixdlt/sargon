import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BIP44LikePathTests: HDPathProtocolTest<BIP44LikePath> {
	func test_sample_description() {
		XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/0H/0/0H")
	}

	func test_sample_from_str() {
		XCTAssertNoDifference(
			"m/44H/1022H/0H/0/0H", // ExpressibleByStringLiteral
			SUT.sample
		)
	}

	func test_invalid_got_cap26_account() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/1H/525H/1460H/0H"))
	}

	func test_index_roundtrip() {
		func doTest(_ sut: SUT) {
			let index = sut.addressIndex
			XCTAssertEqual(SUT(index: index), sut)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_index() throws {
		let sut = try SUT(string: "m/44H/1022H/0H/0/42H")
		XCTAssertEqual(sut.addressIndex, 42)
	}
}
