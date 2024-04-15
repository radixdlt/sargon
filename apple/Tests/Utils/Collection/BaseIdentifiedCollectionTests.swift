import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class BaseCollectionTest<SUT_: BaseIdentifiedCollection>: Test<SUT_> {
	typealias SUTElement = SUT_.Element
	
	func test_element_roundtrip() throws {
		let element = SUTElement.sample
		XCTAssertEqual(
			SUT(element: element).elements,
			[element]
		)
	}
	
	func test_get_id() {
		let element = SUTElement.sample
		let sut = SUT(element: element)
		XCTAssertEqual(sut.get(id: element.id), element)
	}
	
	func test_appending_new()  {
		let sample = SUTElement.sample
		let sampleOther = SUTElement.sampleOther
		let sut = SUT(element: sample)
		XCTAssertEqual(
			sut.appending(sampleOther).elements,
			[sample, sampleOther]
		)
	}
	
	func test_appending_new_duplicate_disallowd()  {
		let sample = SUTElement.sample
		let sut = SUT(element: sample)
		XCTAssertEqual(
			sut.appending(sample).elements, // unchanged
			[sample]
		)
	}
	
	func test_append_filter() {
		let sut = SUT(element: .sample).appending(.sampleOther)
		XCTAssertEqual(
			sut.filter({ $0.id == SUTElement.sample.id }),
			[SUTElement.sample]
		)
	}
}


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
}
