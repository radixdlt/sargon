import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDFromHashTests: Test<FactorSourceIDFromHash> {
	func test_description() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}
	
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSourceID.hash(value: SUT.sample))
	}
	
	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
	
	func test_from_mnemonic_with_passphrase() {
		let sut = SUT(
			kind: .device,
			mnemonicWithPassphrase: .sample
		)
		XCTAssertEqual(sut.toString(), "device:3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240")
	}
	
}

