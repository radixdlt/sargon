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
        eachSample { sut in
            XCTAssertEqual(sut.description, sut.toString())
        }
    }
    
    
    func test_rawValue_is_to_string() {
        eachSample { sut in
            XCTAssertEqual(sut.rawValue, sut.toString())
        }
    }
    
    
    func test_string_roundtrip() throws {
        try eachSample { sut in
            XCTAssertEqual(
                try SUT(string: sut.rawValue),
                sut
            )
        }
    }
}
