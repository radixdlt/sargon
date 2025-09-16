import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class TransferRecipientTests: Test<TransferRecipient> {
	func test_id_is_account_address() {
		XCTAssertEqual(SUT.sample.id, SUT.sample.accountAddress)
	}

	func test_description_is_account_address() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.accountAddress.description)
	}
}
