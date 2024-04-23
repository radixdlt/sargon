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

final class PersonaDataTests: Test<PersonaData> {
	
	func test_entries() {
		XCTAssertEqual(
			SUT.sample.entries,
			[
				.init(id: 1, value: .name(.sample)),
				.init(id: 1, value: .emailAddress(.sample)),
				.init(id: 2, value: .emailAddress(.sampleOther)),
				.init(id: 1, value: .phoneNumber(.sample)),
				.init(id: 2, value: .phoneNumber(.sampleOther)),
			]
		)
	}
	
	func test_default_is_empty() {
		XCTAssertEqual(SUT.default, SUT.init())
	}

}
