import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaDataEntryPhoneNumberTests: PersonaDataEntryTest<PersonaDataEntryPhoneNumber> {
	func test_kind() {
		XCTAssertEqual(SUT.kind, .phoneNumber)
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.kind, .phoneNumber)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_extract_wrong_is_nil() {
		XCTAssertNil(SUT.extract(from: PersonaDataEntryName.sample.embed()))
	}
}
