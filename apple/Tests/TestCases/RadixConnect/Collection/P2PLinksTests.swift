import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class P2PLinksTests: CollectionTest<P2PLink> {
	
	override class func sample() -> SUT {
		SUT.sample
	}
	
	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}
}
