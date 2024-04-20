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
	
	func test_updating_or_appending_new_update() {
		var sample = SUTElement.sample
		let sut = SUT(element: sample)
		sample.displayName = try! DisplayName(validating: "Changed")
		XCTAssertEqual(
			sut.updatingOrAppending(sample).elements,
			[sample]
		)
	}
	
	func test_update_or_append_update() {
		var sample = SUTElement.sample
		var sut = SUT(element: sample)
		sample.displayName = try! DisplayName(validating: "Changed")
		sut.updateOrAppend(sample)
		XCTAssertEqual(sut.elements, [sample])
	}
	
	func test_update_or_append_append() {
		let sample = SUTElement.sample
		var sut = SUT(element: sample)
		sut.updateOrAppend(SUTElement.sampleOther)
		XCTAssertEqual(sut.elements, [sample, .sampleOther])
	}
}
