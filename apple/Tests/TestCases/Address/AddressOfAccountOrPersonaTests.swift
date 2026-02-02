import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AddressOfAccountOrPersonaTests: AddressTest<AddressOfAccountOrPersona> {
	func testAccountAddress() {
		let accountAddress = AccountAddress.sample

		var sut = SUT.account(accountAddress)
		XCTAssertEqual(sut.accountAddress, accountAddress)

		sut = SUT.identity(.sample)
		XCTAssertNil(sut.accountAddress)
	}
}
