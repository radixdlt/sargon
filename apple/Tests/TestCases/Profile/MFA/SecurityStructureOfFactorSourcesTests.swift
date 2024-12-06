import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SecurityStructureOfFactorSourcesTests: Test<SecurityStructureOfFactorSources> {
	func test_id() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.metadata.id)
		}
	}
}
