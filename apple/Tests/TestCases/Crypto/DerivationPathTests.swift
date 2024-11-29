import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DerivationPathTests: HDPathProtocolTest<DerivationPath> {
	func test_sample_description() {
		XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/1H/525H/1460H/0H")
	}

	func test_cap26_account_path_as_derivation_path() {
		let sut = AccountPath.sample

		XCTAssertEqual(
			sut.asGeneral,
			DerivationPath.account(value: sut)
		)
	}

	func test_get_hd_path() {
		eachSample { sut in
			XCTAssertEqual(sut.path.components.count, sut.toString().matches(of: "/").count)
		}
	}

	func test_bip44_account_path_as_derivation_path() {
		let bip44Path = BIP44LikePath.sample

		XCTAssertEqual(
			bip44Path.asGeneral,
			DerivationPath.bip44Like(value: bip44Path)
		)
	}

	func test_bip44_account_path_to_string() {
		let bip44Path = BIP44LikePath.sample

		XCTAssertEqual(
			bip44Path.asDerivationPath.toString(),
			bip44Path.toString()
		)
	}

	func test_bip44_string() throws {
		XCTAssertNoDifference(
			try SUT(string: "m/44H/1022H/0H/0/1H"),
			BIP44LikePath.sampleOther.asGeneral
		)
	}

	func test_as_general_is_identity() {
		XCTAssertEqual(SUT.sample.asDerivationPath, SUT.sample)
	}

	func test_curve() {
		XCTAssertEqual(SUT.sample.curve, .curve25519)
		XCTAssertEqual(SUT.bip44Like(value: .sample).curve, .secp256k1)
	}

	func test_for_entity() throws {
		try XCTAssertEqual(
			SUT.forEntity(
				kind: .account,
				networkID: .mainnet,
				index: .unsecurified(UnsecurifiedHardened(globalKeySpace: 9 + 0x8000_0000))
			)
			.toString(),
			"m/44H/1022H/1H/525H/1460H/9H"
		)
		try XCTAssertEqual(
			SUT.forEntity(
				kind: .persona,
				networkID: .stokenet,
				index: Hardened.unsecurified(UnsecurifiedHardened(localKeySpace: 42))
			)
			.toString(),
			"m/44H/1022H/2H/618H/1460H/42H"
		)
	}
}
