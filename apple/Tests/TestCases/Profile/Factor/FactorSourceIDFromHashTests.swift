import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDFromHashTests: SpecificFactorSourceIDTest<FactorSourceIDFromHash> {
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSourceID.hash(value: SUT.sample))
	}

	func test_from_mnemonic_with_passphrase() {
		let sut = SUT(
			kind: .device,
			mnemonicWithPassphrase: .sample
		)
		XCTAssertEqual(sut.toString(), "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a")
	}

//	func test_extract_wrong_throws() throws {
//		try eachSample { sut in
//			XCTAssertThrowsError(try sut.asGeneral.extract(as: FactorSourceIDFromAddress.self))
//		}
//	}

	func test_spot_check() {
		let input = SpotCheckInput.software(mnemonicWithPassphrase: .sample)
		XCTAssertTrue(SUT.sample.spotCheck(input: input))
	}
}
