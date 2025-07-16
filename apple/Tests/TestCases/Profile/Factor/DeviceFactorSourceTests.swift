import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import SwiftyJSON
import XCTest

final class DeviceFactorSourceTests: SpecificFactorSourceTest<DeviceFactorSource> {
	func test_id_of_device() {
		eachSample { sut in
			XCTAssertEqual(sut.id.description, FactorSourceID.hash(value: sut.id).description)
		}
	}

	func test_factor_source_id_is_id() {
		eachSample { sut in
			XCTAssertEqual(sut.id.asGeneral, sut.factorSourceID)
		}
	}

	func test_kind() {
		eachSample { sut in
			XCTAssertEqual(sut.factorSourceKind, .device)
		}
	}

	func test_as_factor_source_to_string() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.id.description, sut.id.description)
		}
	}

	func test_as_general() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral, FactorSource.device(value: sut))
		}
	}

	func test_new_olympia() {
		let sut = SUT.olympia(mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertTrue(sut.supportsOlympia)
		XCTAssertFalse(sut.supportsBabylon)
	}

	func test_as() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.asDevice, sut)
		}
	}

//	func test_other_wrong() {
//		XCTAssertNil(SUT.extract(from: TrustedContactFactorSource.sample))
//	}

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

	@available(*, deprecated)
	func test_json_decoding_of_profile_fails_if_factorSource_supported_curves_is_empty() throws {
		var json = JSON(Profile.sample)
		json["factorSources"][0]["device.common.cryptoParameters.supportedCurves"] = []
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}
}
