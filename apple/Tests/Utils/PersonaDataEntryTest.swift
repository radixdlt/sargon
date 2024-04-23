//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-21.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class PersonaDataEntryTest<SUT_: PersonaDataEntryProtocol>: Test<SUT_> {
	
	func test_embed_then_extract() throws {
		func doTest(_ sut: SUT) throws {
			let embedded = sut.embed()
			let extracted = try XCTUnwrap(SUT.extract(from: embedded))
			XCTAssertEqual(extracted, sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}
	
	func test_embed_identity()  {
		func doTest(_ sut: SUT)  {
			let embedded = sut.embed()
			XCTAssertEqual(embedded.embed(), embedded)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
	
	func test_formatted_entry() {
		XCTAssertNoDifference(
			SUT.sample.embed().description,
			SUT.sample.description
		)
	}
}
