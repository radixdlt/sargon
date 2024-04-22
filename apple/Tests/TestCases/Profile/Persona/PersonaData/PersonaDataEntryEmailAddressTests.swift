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

final class PersonaDataEntryEmailAddressTests: PersonaDataEntryTest<PersonaDataEntryEmailAddress> {
	
	func test_kind() {
		XCTAssertEqual(SUT.kind, .emailAddress)
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.kind, .emailAddress)
		}
		SUT.allCases.forEach(doTest)
	}
	
	func test_extract_wrong_is_nil() {
		XCTAssertNil(SUT.extract(from: PersonaDataEntryPhoneNumber.sample.embed()))
	}
}
