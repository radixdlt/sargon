import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DetailedAuthorizedPersonasTests: CollectionTest<AuthorizedPersonaDetailed> {
	override class func sample() -> SUT {
		SUT.sample
	}

	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}
}
