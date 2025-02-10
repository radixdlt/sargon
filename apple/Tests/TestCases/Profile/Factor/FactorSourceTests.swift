import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceTests: FactorSourceTest<FactorSource> {
	func test_factor_source_id() {
		XCTAssertEqual(SUT.sample.factorSourceID, SUT.sample.id)
	}

	func test_id_of_device() {
		XCTAssertEqual(SUT.sample.id.description, DeviceFactorSource.sample.id.description)
	}

	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, SUT.sample)
	}

	func test_description() {
		XCTAssertEqual(SUT.sample.toString(), SUT.sample.description)
	}

	func test_factor_source_kind() {
		XCTAssertEqual(SUT.sample.factorSourceKind, .device)
		XCTAssertEqual(SUT.sampleOther.factorSourceKind, .ledgerHqHardwareWallet)
	}

	func test_name() {
		XCTAssertEqual(SUT.sample.name, "My Phone")
	}

	func test_spot_check() {
		let input = SpotCheckInput.software(mnemonicWithPassphrase: .sample)
		XCTAssertTrue(SUT.sample.spotCheck(input: input))
	}
}
