import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaTests: EntityProtocolTest<Persona> {
	
	func test_extract_wrong_throws() {
		func doTest(_ sut: SUT) {
			XCTAssertThrowsError(try sut.asGeneral.extract(as: Account.self))
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_as_general_as_account() throws {
		func doTest(_ sut: SUT) throws {
			XCTAssertEqual(try sut.asGeneral.asPersona(), sut)
		}
		try SUT.sampleValues.forEach(doTest)
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
		XCTAssertEqual(sut.elements, [.deletedByUser])
		XCTAssertEqual(sut, [.deletedByUser])
	}
	
	func test_new() {
		let fi: HierarchicalDeterministicFactorInstance = .sample
		let sut = SUT.init(networkID: .sample, factorInstance: fi, displayName: .sample, extraProperties: .sample)
		XCTAssertEqual(sut.virtualHierarchicalDeterministicFactorInstances, [fi])
	}
}
