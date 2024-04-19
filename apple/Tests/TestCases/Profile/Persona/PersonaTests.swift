import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaTests: EntityTest<Persona> {
	func test_display_names() {
		XCTAssertEqual(
			SUT.allCases.map(\.displayName),
			["Satoshi", "Batman", "Ellen Ripley", "Skywalker", "Granger", "Sarah Connor"]
		)
	}
	
	func test_not_hidden() {
		XCTAssertEqual(SUT.sampleMainnet.flags, [])
	}
	
	func test_hidden() {
		let sut = SUT.sampleMainnetOther.flags
		XCTAssertEqual(sut.elements, [.deletedByUser])
		XCTAssertEqual(sut, [.deletedByUser])
	}
}
