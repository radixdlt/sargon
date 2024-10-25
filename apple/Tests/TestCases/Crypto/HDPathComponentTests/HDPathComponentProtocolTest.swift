//
//  HDPathComponentProtocolTest.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import XCTest
import Sargon


class HDPathComponentProtocolTest<SUT_: HDPathComponentProtocol>: BaseHDPathComponentProtocolTest<SUT_> {
	func test_local() throws {
		for local in UInt32(0)...10 {
			let sut = try SUT(localKeySpace: local)
			let indexInLocal = sut.indexInLocalKeySpace()
			XCTAssertEqual(local, indexInLocal)
			XCTAssertEqual(try SUT(localKeySpace: indexInLocal), sut)
		}
	}
	
	
	func test_local_to_global() throws {
		let sut = try SUT(localKeySpace: 42)
		let global = sut.indexInGlobalKeySpace()
		let fromGlobal = try SUT(globalKeySpace: global)
		XCTAssertEqual(fromGlobal, sut)
	}
	
	func test_global() throws {
		for global in SUT.globalOffset ... SUT.globalOffset + 3 {
			let sut = try SUT(globalKeySpace: global)
			let indexInGlobal = sut.indexInGlobalKeySpace()
			XCTAssertEqual(global, indexInGlobal)
			XCTAssertEqual(try SUT(globalKeySpace: indexInGlobal), sut)
		}
	}
}


