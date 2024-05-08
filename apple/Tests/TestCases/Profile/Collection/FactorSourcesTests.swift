import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourcesTests: CollectionTest<FactorSource> {

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
	func omit_crash_if_empty() {
		var profile = Profile.sample
		profile.factorSources = [] // empty FactorSources is not allowed
		let _ = profile.jsonData() // should crash
	}
}
