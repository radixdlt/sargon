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
		try eachSample { sut in
			let embedded = sut.embed()
			let extracted = try XCTUnwrap(SUT.extract(from: embedded))
			XCTAssertEqual(extracted, sut)
		}
	}
	
	func test_embed_identity()  {
		eachSample { sut in
			let embedded = sut.embed()
			XCTAssertEqual(embedded.embed(), embedded)
		}
	}
	
	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more 
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
	
	func test_formatted_entry() {
		XCTAssertNoDifference(
			SUT.sample.embed().description,
			SUT.sample.description
		)
	}
}
