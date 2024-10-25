//
//  UnhardenedTests.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import XCTest
import Sargon

final class UnhardenedTests: HDPathComponentProtocolTest<Unhardened> {
	
	func test_fromU31() throws {
		let sut = try SUT(u31: U31(value: 5))
		try XCTAssertEqual(SUT(localKeySpace: 5), sut)
	}
}

