//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class LedgerHardwareWalletModelTests: Test<LedgerHardwareWalletModel> {

    func test_description_is_to_string() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(sut.description, sut.toString())
        }
        SUT.allCases.forEach(doTest)
    }
    
    
    func test_rawValue_is_to_string() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(sut.rawValue, sut.toString())
        }
        SUT.allCases.forEach(doTest)
    }
    
    
    func test_string_roundtrip() throws {
        func doTest(_ sut: SUT) throws {
            XCTAssertEqual(
                try SUT(string: sut.rawValue),
                sut
            )
        }
        try SUT.allCases.forEach(doTest)
    }
}
