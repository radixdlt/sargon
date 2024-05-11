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
		XCTAssertEqual(sut.toString(), "device:3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240")
	}
	
	func test_extract_wrong_throws() throws {
		try eachSample { sut in
			XCTAssertThrowsError(try sut.asGeneral.extract(as: FactorSourceIDFromAddress.self))
		}
	}
}

