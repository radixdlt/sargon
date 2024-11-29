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

	func test_show_deposits() {
		var sut = SUT.sample
		XCTAssertTrue(sut.isDepositsVisible)
		sut.showDeposits(false)
		XCTAssertFalse(sut.isDepositsVisible)
	}
}
