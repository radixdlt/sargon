//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-06.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SecurityStructureMetadataTests: Test<SecurityStructureMetadata> {
    func test_new_with_name() {
        let sut = SUT.init(name: "foo")
        XCTAssertEqual(sut.displayName, "foo")
    }
}
