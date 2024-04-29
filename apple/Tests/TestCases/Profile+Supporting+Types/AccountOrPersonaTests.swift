import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AccountOrPersonaTests: EntityBaseTest<AccountOrPersona> {
	func test_display_names() {
		XCTAssertEqual(SUT.sampleValues.map(\.displayName), ["Alice", "Batman", "Carol", "Nadia", "Granger", "Paige"])
	}

	func test_as_general_is_self() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut, sut.asGeneral)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
