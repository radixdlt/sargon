import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AssetsExceptionListTests: CollectionTest<AssetException> {
	override class func sample() -> SUT {
		SUT.sample
	}

	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}
}
