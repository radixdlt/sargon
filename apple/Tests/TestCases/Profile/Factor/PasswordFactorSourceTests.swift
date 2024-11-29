import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PasswordFactorSourceTests: SpecificFactorSourceTest<PasswordFactorSource> {
	func test_id_of_password() {
		eachSample { sut in
			XCTAssertEqual(sut.id.description, FactorSourceID.hash(value: sut.id).description)
		}
	}

	func test_new() {
		XCTAssertEqual(
			SUT(
				mnemonicWithPassphrase: .init(
					mnemonic: .samplePassword,
					passphrase: ""
				),
				hint: .init(label: "Password 1")
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
			XCTAssertEqual(sut.factorSourceKind, .password)
		}
	}

	func test_as() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.asPassword, sut)
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
			XCTAssertEqual(sut.asGeneral, FactorSource.password(value: sut))
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
