//
//  HDPathComponentTests.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-24.
//

import Sargon
import XCTest

extension KeySpace: CaseIterable {
    public static var allCases: [KeySpace] {
        [.securified, .unsecurified(isHardened: false), .unsecurified(isHardened: true)]
    }
}

final class HDPathComponentTests: BaseHDPathComponentProtocolTest<HdPathComponent> {
    
    func test_local_roundtrip() throws {
		for keySpace in KeySpace.allCases {
			for local in UInt32(0)...3 {
				let sut = try SUT(localKeySpace: local, keySpace: keySpace)
				let indexInLocal = sut.indexInLocalKeySpace()
				XCTAssertEqual(local, indexInLocal)
				XCTAssertEqual(try SUT(localKeySpace: indexInLocal, keySpace: keySpace), sut)
			}
		}
    
    }
    
    func test_global_roundtrip() throws {
		struct Param {
			let keySpace: KeySpace
			let global: UInt32
			let description: String
			let debugDescription: String
			init(_ keySpace: KeySpace, _ global: UInt32, _ description: String, _ debugDescription: String) {
				self.keySpace = keySpace
				self.global = global
				self.description = description
				self.debugDescription = debugDescription
			}
		}
		
		[
			Param(KeySpace.unsecurified(isHardened: false), 9, "9", "9"),
			Param(KeySpace.unsecurified(isHardened: true), 0x80000009, "9'", "9H"),
			Param(KeySpace.securified, 0xc0000009, "9^", "9S")
		].forEach { params in
			let keySpace = params.keySpace
			let global = params.global
			let description = params.description
			let debugDescription = params.debugDescription
			
			let sut = SUT(globalKeySpace: global)
			XCTAssertEqual(sut.indexInLocalKeySpace(), 9)
			XCTAssertEqual(sut.keySpace, keySpace)
			XCTAssertEqual(sut.indexInGlobalKeySpace(), global)
			XCTAssertEqual(sut.description, description)
			XCTAssertEqual(sut.debugDescription, debugDescription)
		}
		
 
    }
    
    
    
}
