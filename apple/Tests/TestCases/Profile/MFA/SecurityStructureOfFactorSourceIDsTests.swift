import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SecurityStructureOfFactorSourceIDsTests: Test<SecurityStructureOfFactorSourceIDs> {
	func test_id() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.metadata.id)
		}
	}
}
