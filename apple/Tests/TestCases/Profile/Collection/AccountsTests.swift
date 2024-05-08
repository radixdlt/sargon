import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

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
}
