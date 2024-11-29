import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SecurityStructureMetadataTests: Test<SecurityStructureMetadata> {
	func test_new_with_name() {
		let sut = SUT(name: "foo")
		XCTAssertEqual(sut.displayName, "foo")
	}
}
