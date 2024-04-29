import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AuthorizedDappDetailedTests: Test<AuthorizedDappDetailed> {
	func test_id_is_dappDefinitionAddress() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.dappDefinitionAddress)
		}
	}
}
