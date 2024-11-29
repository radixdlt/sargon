import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AccountPathTests: HDPathProtocolTest<AccountPath> {
	func test_sample_description() {
		XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/1H/525H/1460H/0H")
	}

	func test_sample_from_str() {
		XCTAssertNoDifference(
			"m/44H/1022H/1H/525H/1460H/0H",
			SUT.sample
		)
	}

	func test_invalid_got_identity() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/1H/618H/1460H/0H"))
	}

	func test_invalid_got_bip44_like_legacy_path() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/0H/0/0H"))
	}

	func test_init_network_id_key_kind_index() throws {
		try XCTAssertEqual(
			SUT.sampleOther,
			SUT(
				networkID: .mainnet,
				keyKind: .transactionSigning,
				index: .unsecurified(UnsecurifiedHardened(localKeySpace: 1))
			)
		)
	}

	func test_index() throws {
		XCTAssertEqual(
			SUT.sample.asGeneral.lastPathComponent,
			HdPathComponent(globalKeySpace: 0 + 0x8000_0000)
		)

		try XCTAssertEqual(
			SUT.sampleOther.asGeneral.lastPathComponent,
			HdPathComponent(localKeySpace: 1, keySpace: .unsecurified(isHardened: true))
		)
	}
}
