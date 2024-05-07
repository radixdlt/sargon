import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class AccountsTests: Test<Accounts> {
    /*
	func test_accounts_count() {
		var sut: SUT = []
		func doTest(expected: Int) {
			XCTAssertEqual(expected, sut.count)
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
		sample.displayName = "New Name"
		let elem = sut.updateOrAppend(sample)
		XCTAssertEqual(sut.elements, [sample])
		XCTAssertEqual(elem, SUTElement.sample)
	}
	
	func test_update_or_append_append() {
		let sample = SUTElement.sample
		var sut = SUT(element: sample)
		sut.updateOrAppend(SUTElement.sampleOther)
		XCTAssertEqual(sut.elements, [sample, .sampleOther])
	}
	
	
	func test_updateOrInsert_exists_wrong_index() {
		var sut: SUT = [.sample, .sampleOther]

		let sampleChanged: SUTElement = {
			var a = SUTElement.sample
			a.displayName = "SampleChanged"
			return a
		}()
		let wrongIndex = 1
		let (originalMember, deFactoIndex) = sut.updateOrInsert(element: sampleChanged, at: wrongIndex) // wrong index
		XCTAssertEqual(originalMember, .sample)
		XCTAssertNotEqual(deFactoIndex, wrongIndex)
		XCTAssertEqual(deFactoIndex, 0)
		XCTAssertEqual(sut.elements, [sampleChanged, .sampleOther])
	}
	
	func test_replaceSubrange_first() {
		var sut: SUT = [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia]
		sut.replaceSubrange(0..<1, with: [.sampleStokenetOlivia])
		XCTAssertEqual(sut, [.sampleStokenetOlivia, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia])
	}
	
	func test_replaceSubrange_many_leading() {
		var sut: SUT = [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia]
		sut.replaceSubrange(0...1, with: [.sampleStokenetPaige, .sampleStokenetOlivia])
		XCTAssertEqual(sut, [.sampleStokenetPaige, .sampleStokenetOlivia, .sampleMainnetCarol, .sampleStokenetNadia])
	}
	
	func test_replaceSubrange_many_middle() {
		var sut: SUT = [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia]
		sut.replaceSubrange(1...2, with: [.sampleStokenetPaige, .sampleStokenetOlivia])
		XCTAssertEqual(sut, [.sampleMainnetAlice, .sampleStokenetPaige, .sampleStokenetOlivia, .sampleStokenetNadia])
	}
	
	func test_replaceSubrange_many_trailing() {
		var sut: SUT = [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia]
		sut.replaceSubrange(2...3, with: [.sampleStokenetPaige, .sampleStokenetOlivia])
		XCTAssertEqual(sut, [.sampleMainnetAlice, .sampleMainnetBob, .sampleStokenetPaige, .sampleStokenetOlivia])
	}
	
	func test_replaceSubrange_last() {
		var sut: SUT = [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia]
		sut.replaceSubrange(3..<4, with: [.sampleStokenetOlivia])
		XCTAssertEqual(sut, [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetOlivia])
	}
	
	func test_replaceSubrange_last_many() {
		var sut: SUT = [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetNadia]
		sut.replaceSubrange(3..<4, with: [.sampleStokenetOlivia, .sampleStokenetPaige])
		XCTAssertEqual(sut, [.sampleMainnetAlice, .sampleMainnetBob, .sampleMainnetCarol, .sampleStokenetOlivia, .sampleStokenetPaige])
	}
	
	func test_updateOrInsert_exists_correct_index() {
		var sut: SUT = [.sample, .sampleOther]

		let sampleChanged: SUTElement = {
			var a = SUTElement.sample
			a.displayName = "SampleChanged"
			return a
		}()
		let correctIndex = 0
		let (originalMember, deFactoIndex) = sut.updateOrInsert(element: sampleChanged, at: correctIndex) // correct index
		XCTAssertEqual(originalMember, .sample)
		XCTAssertEqual(deFactoIndex, correctIndex)
		XCTAssertEqual(sut.elements, [sampleChanged, .sampleOther])
	}
     */
}
