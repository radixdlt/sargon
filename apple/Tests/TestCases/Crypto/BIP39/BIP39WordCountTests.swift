import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BIP39WordCountTests: Test<BIP39WordCount> {
	func test_all_cases() {
		XCTAssertEqual(SUT.allCases, [.twentyFour, .twentyOne, .eighteen, .fifteen, .twelve])
	}
	
	func test_id_is_raw_value() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.rawValue)
		}
	}
	
	func test_init_raw_value() {
		eachSample { sut in
			XCTAssertEqual(SUT.init(rawValue: sut.rawValue), sut)
		}
	}
	
	func test_init_wordCount() {
		eachSample { sut in
			XCTAssertEqual(SUT.init(wordCount: Int(sut.rawValue)), sut)
		}
	}
	
	func test_comparable_less_than() {
		XCTAssertLessThan(SUT.eighteen, SUT.twentyOne)
		XCTAssertLessThan(SUT.eighteen, SUT.twentyFour)
	}
	
	func test_comparable_greater_than() {
		XCTAssertGreaterThan(SUT.eighteen, SUT.fifteen)
		XCTAssertGreaterThan(SUT.fifteen, SUT.twelve)
	}
	
	func test_decrease() {
		var sut = SUT.fifteen
		sut.decreaseBy3()
		XCTAssertEqual(sut, .twelve)
	}
	
	func test_increase() {
		var sut = SUT.twentyOne
		sut.increaseBy3()
		XCTAssertEqual(sut, .twentyFour)
	}
}
