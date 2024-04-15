import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class AccountsTests: CanBeEmptyCollectionTest<Accounts> {
	func test_accounts_count() {
		var sut: SUT = []
		func doTest(expected: Int) {
			XCTAssertEqual(expected, sut.count)
			XCTAssertEqual(sut.elementCount, sut.elements.count)
			XCTAssertEqual(sut.elementCount, sut.count)
		}
		doTest(expected: 0)
		
		sut.append(.sampleMainnet)
		doTest(expected: 1)
		
		sut.append(.sampleMainnet)
		doTest(expected: 1) // duplicates prevented, still `1`
		
		sut.append(.sampleMainnetOther)
		doTest(expected: 2)
		
		sut.append(.sampleMainnetThird)
		doTest(expected: 3)

	}
}
