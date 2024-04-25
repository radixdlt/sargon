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
			sut.removing(SUTElement.sample.id),
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
		let removed = sut.remove(SUTElement.sample.id)
		XCTAssertEqual(removed, SUTElement.sample)
		XCTAssertEqual(
			sut,
			[.sampleOther]
		)
	}
	
	func test_init() {
		XCTAssertEqual(SUT.init(), SUT.init([]))
		XCTAssertEqual(SUT.init().count, 0)
	}
	
	func test_replaceSubrange() {
		var sut = [SUTElement.sample]
		sut.replaceSubrange(0..<1, with: [SUTElement.sampleOther])
		XCTAssertEqual(sut, [SUTElement.sampleOther])
	}
	
	func test_remove_element_by_id_not_present_returns_nil() {
		var sut: SUT = [.sampleOther]
		XCTAssertNil(sut.remove(SUTElement.sample.id))
		XCTAssertEqual(sut, [.sampleOther])
	}
	
	func test_remove_by_element() {
		var sut: SUT = [.sample, .sampleOther]
		let removed = sut.remove(element: .sample)
		XCTAssertEqual(removed, SUTElement.sample)
		XCTAssertEqual(
			sut,
			[.sampleOther]
		)
	}
	
	func test_remove_element_by_element_not_present_returns_nil() {
		var sut: SUT = [.sampleOther]
		XCTAssertNil(sut.remove(element: SUTElement.sample))
		XCTAssertEqual(sut, [.sampleOther])
	}
	
	func test_subscript_id_get() {
		let sut: SUT = [.sample, .sampleOther]
		XCTAssertEqual(sut[id: SUTElement.sample.id], SUTElement.sample)
	}
	
	func test_remove_by_id_subscript() {
		var sut: SUT = [.sample, .sampleOther]
		sut[id: SUTElement.sample.id] = nil
		XCTAssertEqual(
			sut,
			[.sampleOther]
		)
	}
	
	func test_add_by_id_subscript() {
		var sut: SUT = [.sample]
		sut[id: SUTElement.sampleOther.id] = SUTElement.sampleOther
		XCTAssertEqual(
			sut,
			[.sample, .sampleOther]
		)
	}
}




















