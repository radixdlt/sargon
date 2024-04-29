import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SharedPersonaDataTests: Test<SharedPersonaData> {
	func test_entryIDs() {
		XCTAssertEqual(
			SUT.sample.entryIDs,
			[0, 1, 2, 3, 4]
		)
	}

	func test_entryIDs_empty() {
		XCTAssertEqual(SUT.default.entryIDs, [])
	}

	func test_default_is_empty() {
		XCTAssertEqual(
			SUT.default,
			.init(
				name: nil,
				emailAddresses: nil,
				phoneNumbers: nil
			)
		)
	}
}
