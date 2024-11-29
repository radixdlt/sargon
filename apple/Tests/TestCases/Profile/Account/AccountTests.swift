import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AccountTests: EntityProtocolTest<Account> {
	func test_extract_wrong_throws() throws {
		try eachSample { sut in
			XCTAssertThrowsError(try sut.asGeneral.extract(as: Persona.self))
		}
	}

	func test_as_general_as_account() throws {
		try eachSample { sut in
			XCTAssertEqual(try sut.asGeneral.asAccount(), sut)
		}
	}

	func test_display_names() {
		XCTAssertEqual(SUT.sampleValues.map(\.displayName), ["Alice", "Bob", "Carol", "Nadia", "Olivia", "Paige"])
	}

	func test_not_hidden() {
		XCTAssertEqual(SUT.sampleMainnetAlice.flags, [])
	}

	func test_hidden() {
		let sut = SUT.sampleStokenetOlivia.flags
		XCTAssertEqual(sut, [.hiddenByUser])
	}

	func test_appearance_id() {
		XCTAssertEqual(SUT.sampleMainnetAlice.appearanceID, AppearanceID(value: 0))
		XCTAssertEqual(SUT.sampleMainnetBob.appearanceID, AppearanceID(value: 1))
	}

	func test_ledger_controlled_account_has_no_device_fs_id() {
		var sut = SUT.sample
		var uec = UnsecuredEntityControl.sample
		uec.transactionSigning.factorSourceId = .init(kind: .ledgerHqHardwareWallet, body: .sample)
		sut.securityState = .unsecured(value: uec)
		XCTAssertNil(sut.deviceFactorSourceID)
	}

	func test_virtual_hd_deterministic_factor_instances_includes_auth_signing_if_set() {
		var sut = SUT.sample
		sut.securityState = .unsecured(value: .init(transactionSigning: .sample, authenticationSigning: .sampleOther))
		XCTAssertEqual(sut.virtualHierarchicalDeterministicFactorInstances.count, 2)
	}

	func test_new() {
		let fi: HierarchicalDeterministicFactorInstance = .sample
		let sut = SUT(networkID: .sample, factorInstance: fi, displayName: .sample, extraProperties: .sample)
		XCTAssertEqual(sut.virtualHierarchicalDeterministicFactorInstances, [fi])
	}
}
