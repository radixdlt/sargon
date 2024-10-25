//
//  HardenedTests.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import XCTest
import Sargon

final class HardenedTests: Test<Hardened> {
    func test_() {
        XCTAssertEqual(
            SUT.sample,
            SUT.securified(.sample)
        )
    }
}
