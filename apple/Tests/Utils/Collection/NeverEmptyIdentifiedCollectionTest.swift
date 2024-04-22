import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class NeverEmptyIdentifiedCollectionTest<SUT_: NeverEmptyIdentifiedCollection>: BaseCollectionTest<SUT_> {
	
	func test_cannot_be_empty() {
		XCTAssertThrowsError(try SUT([]))
	}
	
	func test_removing_element_by_id() throws {
		let sut = try SUT([.sample, .sampleOther])
		XCTAssertEqual(
			try sut.removing(SUTElement.sample.id),
			try SUT([.sampleOther])
		)
	}
	
	func test_removing_by_element() throws {
		let sut = try SUT([.sample, .sampleOther])
		XCTAssertEqual(
			try sut.removing(element: SUTElement.sample),
			try SUT([.sampleOther])
		)
	}

	func test_removing_element_by_id_throws_if_single_element() throws {
		let sut = try SUT([.sample])
		XCTAssertThrowsError(try sut.removing(SUTElement.sample.id))
	}
	
	func test_removing_by_element_throws_if_single_element() throws {
		let sut = try SUT([.sample])
		XCTAssertThrowsError(try sut.removing(element: SUTElement.sample))
	}
	
	func test_remove_element_by_id() throws {
		var sut = try SUT([.sample, .sampleOther])
		try sut.remove(SUTElement.sample.id)
		XCTAssertEqual(
			sut,
			try SUT([.sampleOther])
		)
	}
	
	func test_remove_by_element() throws {
		var sut = try SUT([.sample, .sampleOther])
		try sut.remove(element: .sample)
		XCTAssertEqual(
			sut,
			try SUT([.sampleOther])
		)
	}
}
