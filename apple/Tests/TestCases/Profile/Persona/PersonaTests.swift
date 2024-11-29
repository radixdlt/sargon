import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaTests: EntityProtocolTest<Persona> {
	func test_extract_wrong_throws() throws {
		try eachSample { sut in
			XCTAssertThrowsError(try sut.asGeneral.extract(as: Account.self))
		}
	}

	func test_as_general_as_account() throws {
		try eachSample { sut in
			XCTAssertEqual(try sut.asGeneral.asPersona(), sut)
		}
	}

	func test_display_names() {
		XCTAssertEqual(
			SUT.sampleValues.map(\.displayName),
			["Satoshi", "Batman", "Ellen Ripley", "Skywalker", "Granger", "Sarah Connor"]
		)
	}

	func test_not_hidden() {
		XCTAssertEqual(SUT.sampleMainnet.flags, [])
	}

	func test_hidden() {
		let sut = SUT.sampleMainnetTuring.flags
		XCTAssertEqual(sut, [.hiddenByUser])
	}

	func test_new() {
		let fi: HierarchicalDeterministicFactorInstance = .sample
		let sut = SUT(networkID: .sample, factorInstance: fi, displayName: .sample, extraProperties: .sample)
		XCTAssertEqual(sut.virtualHierarchicalDeterministicFactorInstances, [fi])
	}
}
