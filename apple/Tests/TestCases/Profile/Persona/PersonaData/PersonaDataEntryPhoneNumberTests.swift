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

final class PersonaDataEntryPhoneNumberTests: PersonaDataEntryTest<PersonaDataEntryPhoneNumber> {
	
	func test_kind() {
		XCTAssertEqual(SUT.kind, .phoneNumber)
		eachSample { sut in
			XCTAssertEqual(sut.kind, .phoneNumber)
		}
	}
	
	func test_extract_wrong_is_nil() {
		XCTAssertNil(SUT.extract(from: PersonaDataEntryName.sample.embed()))
	}
	
}
