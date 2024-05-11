import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest
import SwiftyJSON

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
	
	func test_extract_wrong_throws() throws {
		try eachSample { sut in
			XCTAssertThrowsError(try sut.asGeneral.extract(as: LedgerHardwareWalletFactorSource.self))
		}
	}
	
	func test_known_factor_source_id() async throws {
		let mnemonic = try Mnemonic(
			phrase: "equip will roof matter pink blind book anxiety banner elbow sun young"
		)
		
		let factorSourceID = FactorSourceIDFromHash(
			kind: .device,
			mnemonicWithPassphrase: .init(
				mnemonic: mnemonic,
				passphrase: "Radix... just imagine!"
			)
		)
		
		XCTAssertEqual(
			factorSourceID.description,
			"device:4af22ea955d53263a712d897a797df8388e13b8e7b3f30d7d7da88028b724d60"
		)
	}
	
	func test_json_decoding_of_profile_fails_if_factorSource_supported_curves_is_empty() throws {
		var json = JSON(Profile.sample)
		json["factorSources"][0]["device.common.cryptoParameters.supportedCurves"] = []
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}
}
