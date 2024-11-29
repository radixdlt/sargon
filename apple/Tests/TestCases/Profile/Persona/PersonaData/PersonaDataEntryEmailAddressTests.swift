import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaDataEntryEmailAddressTests: PersonaDataEntryTest<PersonaDataEntryEmailAddress> {
	func test_kind() {
		XCTAssertEqual(SUT.kind, .emailAddress)
		eachSample { sut in
			XCTAssertEqual(sut.kind, .emailAddress)
		}
	}

	func test_extract_wrong_is_nil() {
		XCTAssertNil(SUT.extract(from: PersonaDataEntryPhoneNumber.sample.embed()))
	}
}
