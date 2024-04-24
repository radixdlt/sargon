import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DeviceFactorSourceTests: SpecificFactorSourceTest<DeviceFactorSource> {
	func test_id_of_device() {
		XCTAssertEqual(SUT.sample.id.description, FactorSourceID.hash(value: SUT.sample.id).description)
	}
	
	func test_factor_source_id_is_id() {
		XCTAssertEqual(SUT.sample.id.asGeneral, SUT.sample.factorSourceID)
	}
	
	func test_kind() {
		XCTAssertEqual(SUT.sample.factorSourceKind, .device)
	}
	
	func test_as_factor_source_to_string() {
		XCTAssertEqual(SUT.sample.asGeneral.id.description, SUT.sample.id.description)
	}
	
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSource.device(value: SUT.sample))
	}
	
	func test_new_babylon_is_main_true() {
		let sut = SUT.babylon(mnemonicWithPassphrase: .sample, isMain: true)
		XCTAssertTrue(sut.isMainBDFS)
	}
	
	func test_new_babylon_is_main_false() {
		let sut = SUT.babylon(mnemonicWithPassphrase: .sample, isMain: false)
		XCTAssertFalse(sut.isMainBDFS)
	}
	
	func test_new_babylon() {
		let sut = SUT.babylon(mnemonicWithPassphrase: .sample, isMain: true)
		XCTAssertTrue(sut.supportsBabylon)
		XCTAssertFalse(sut.supportsOlympia)
	}
	
	func test_new_olympia() {
		let sut = SUT.olympia(mnemonicWithPassphrase: .sample)
		XCTAssertTrue(sut.supportsOlympia)
		XCTAssertFalse(sut.supportsBabylon)
	}
	
	func test_extract_wrong_throws() {
		func doTest(_ sut: SUT) {
			XCTAssertThrowsError(try sut.asGeneral.extract(as: LedgerHardwareWalletFactorSource.self))
		}
		SUT.sampleValues.forEach(doTest)
	}
}
