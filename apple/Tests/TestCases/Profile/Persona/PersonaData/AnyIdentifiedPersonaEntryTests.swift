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

final class AnyIdentifiedPersonaEntryTests: Test<AnyIdentifiedPersonaEntry> {
	
	func test_PersonaDataIdentifiedEntry_description() {
		let number = "123456789"
		let sut = SUT(
			id: 1,
			value: PersonaData.PhoneNumber(number: number).embed()
		)
		
		XCTAssertNoDifference(
			sut.description,
			"""
			\(number)
			id: 00000000-0000-0000-0000-000000000001
			"""
		)
	}
}
