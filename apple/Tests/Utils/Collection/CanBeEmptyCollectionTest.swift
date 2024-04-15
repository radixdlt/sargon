import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class CanBeEmptyCollectionTest<SUT_: CanBeEmptyIdentifiedCollection>: BaseCollectionTest<SUT_> {
	func test_can_be_empty() {
		XCTAssertEqual(SUT([]).count, 0)
	}
	
	func test_expressible_by_array_literal() {
		XCTAssertEqual(SUT(element: SUTElement.sample), [SUTElement.sample])
	}
	
	func test_removing_element_by_id() {
		let sut: SUT = [.sample, .sampleOther]
		XCTAssertEqual(
			sut.removingElementByID(SUTElement.sample.id),
			[.sampleOther]
		)
	}
	
	func test_removing_by_element() {
		let sut: SUT = [.sample, .sampleOther]
		XCTAssertEqual(
			sut.removing(element: SUTElement.sample),
			[.sampleOther]
		)
	}
	
	func test_remove_element_by_id() {
		var sut: SUT = [.sample, .sampleOther]
		sut.removeElementByID(SUTElement.sample.id)
		XCTAssertEqual(
			sut,
			[.sampleOther]
		)
	}
	
	func test_remove_by_element() {
		var sut: SUT = [.sample, .sampleOther]
		sut.remove(element: .sample)
		XCTAssertEqual(
			sut,
			[.sampleOther]
		)
	}
}




















