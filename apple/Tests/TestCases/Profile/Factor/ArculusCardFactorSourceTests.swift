import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ArculusCardFactorSourceTests: SpecificFactorSourceTest<ArculusCardFactorSource> {
	func test_id_of_acrulus() {
		eachSample { sut in
			XCTAssertEqual(sut.id.description, FactorSourceID.hash(value: sut.id).description)
		}
	}

	func test_new() {
		XCTAssertEqual(
			SUT(
				mnemonicWithPassphrase: .init(
					mnemonic: .sampleArculus,
					passphrase: ""
				),
				label: "Test"
			).id,
			SUT.sample.id
		)
	}

	func test_factor_source_id_is_id() {
		eachSample { sut in
			XCTAssertEqual(sut.id.asGeneral, sut.factorSourceID)
		}
	}

	func test_kind() {
		eachSample { sut in
			XCTAssertEqual(sut.factorSourceKind, .arculusCard)
		}
	}

	func test_as() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.asArculus, sut)
		}
	}

	func test_other_wrong() {
		XCTAssertNil(SUT.extract(from: DeviceFactorSource.sample))
	}

	func test_as_factor_source_to_string() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.id.description, sut.id.description)
		}
	}

	func test_as_general() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral, FactorSource.arculusCard(value: sut))
		}
	}

	func test_source_that_supports_babylon() {
		let sut = SUT.sample
		XCTAssertTrue(sut.supportsBabylon)
		XCTAssertFalse(sut.supportsOlympia)
	}

	func test_extract_wrong_throws() throws {
		try eachSample { sut in
			XCTAssertThrowsError(try sut.asGeneral.extract(as: DeviceFactorSource.self))
		}
	}
}
