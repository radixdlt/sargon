import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class GatewaysTests: CollectionTest<Gateway> {
	override class func sample() -> SUT {
		SUT.sample
	}

	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}
}
