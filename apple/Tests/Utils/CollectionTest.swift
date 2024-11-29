import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class CollectionTest<Element: SargonModel & Identifiable>: TestCase {
	typealias SUT = [Element]

	class func sample() -> SUT {
		fatalError("override me")
	}

	class func sampleOther() -> SUT {
		fatalError("override me")
	}

	func test_equality() {
		XCTAssertNoDifference(Self.sample(), Self.sample())
	}

	func test_inequality() {
		XCTAssertNotEqual(Self.sample(), Self.sampleOther())
	}

	func test_ids_are_unique() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(
				Set(sut.map(\.id)).count,
				Set(sut).count
			)
		}
		doTest(Self.sample())
		doTest(Self.sampleOther())
	}
}
