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
	
	func test_updating_or_appending_new_append() {
		let sample = SUTElement.sample
		let sampleOther = SUTElement.sampleOther
		let sut = SUT(element: sample)
		XCTAssertEqual(
			sut.updatingOrAppending(sampleOther).elements,
			[sample, sampleOther]
		)
	}
	
	func test_append_new()  {
		let sample = SUTElement.sample
		let sampleOther = SUTElement.sampleOther
		var sut = SUT(element: sample)
		sut.append(sampleOther)
		XCTAssertEqual(
			sut.elements,
			[sample, sampleOther]
		)
	}
	
	func test_count()  {
		let sample = SUTElement.sample
		let sampleOther = SUTElement.sampleOther
		var sut = SUT(element: sample)
		XCTAssertEqual(sut.count, 1)
		sut.append(sampleOther)
		XCTAssertEqual(sut.count, 2)
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
