import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest
import SwiftyJSON

final class AccountsTests: CollectionTest<Account> {
	override class func sample() -> SUT {
		SUT.sample
	}
	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}
	
	/// Have to omit this test... obviously... since it crashes.
	/// We can have this test implemented when swift-testing is stable to be used,
	/// and we will use "exit tests" to test it:
	/// https://forums.swift.org/t/exit-tests-death-tests-and-you/71186
	func omit_crash_if_duplicates() { // this test is relevant for Personas, AuthorizedDapps, ProfileNetworks etc etc... they all use the same rust type, which does not allow duplicates
		var profile = Profile.sample
		let a = Account.sample
		var b = a
		b.displayName = "Diff name, also crash" // different value on the element does not affect duplicates check, since it is ID based
		profile.networks[0].accounts = [a, b] // Duplicates (by ID), not allowed => crash
		let _ = profile.jsonData() // should crash
	}

	func test_json_decoding_of_profile_fails_if_accounts_contains_duplicates() throws {
		var json = JSON(Profile.sample)
		json["profileNetworks.accounts"] = [Account.sample, Account.sample]
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}
	
	func test_json_decoding_of_profile_fails_if_accounts_contains_duplicated_ids() throws {
		var json = JSON(Profile.sample)
		let a = Account.sample
		var b = a
		b.displayName = "Diff name, also crash" // different value on the element does not affect duplicates check, since it is ID based
		json["profileNetworks.accounts"] = [a, b]
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}
}
