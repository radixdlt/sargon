//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaDataEntryTests: Test<PersonaData.Entry> {
	func test_discriminator() {
		XCTAssertEqual(SUT.name(.sample).discriminator, .fullName)
		XCTAssertEqual(SUT.emailAddress(.sample).discriminator, .emailAddress)
		XCTAssertEqual(SUT.phoneNumber(.sample).discriminator, .phoneNumber)
	}
	
	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more 
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
	
	func test_embed_extract_name() throws {
		let value: PersonaDataEntryName = .sample
		let sut: SUT = value.embed()
		try XCTAssertEqual(sut.extract(), value)
		XCTAssertThrowsError(try sut.extract(as: PersonaDataEntryPhoneNumber.self))
	}
	
	func test_embed_extract_email() throws {
		let value: PersonaDataEntryEmailAddress = .sample
		let sut: SUT = value.embed()
		try XCTAssertEqual(sut.extract(), value)
		XCTAssertThrowsError(try sut.extract(as: PersonaDataEntryName.self))
	}
	
	func test_embed_extract_phone() throws {
		let value: PersonaDataEntryPhoneNumber = .sample
		let sut: SUT = value.embed()
		try XCTAssertEqual(sut.extract(), value)
		XCTAssertThrowsError(try sut.extract(as: PersonaDataEntryEmailAddress.self))
	}
}
