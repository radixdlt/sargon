//
//  HDPathComponentTests.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-24.
//

import Sargon
import Testing

extension KeySpace: CaseIterable {
	public static var allCases: [KeySpace] {
		[.securified, .unsecurified(isHardened: false), .unsecurified(isHardened: true)]
	}
}

@Suite
struct HDPathComponentTests {
	typealias Sut = HdPathComponent

	@Test("Local roundtrip", arguments: KeySpace.allCases)
	func local_roundtrip(keySpace: KeySpace) throws {
		for local in UInt32(0)...3 {
			let sut = try Sut(indexInLocalKeySpace: local, keySpace: keySpace)
			let indexInLocal = sut.indexInLocalKeySpace()
			#expect(local == indexInLocal)
			#expect(try Sut(indexInLocalKeySpace: indexInLocal, keySpace: keySpace) == sut)
		}
	}
	
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
	
	@Test(
		"Global roundtrip",
		arguments: [
			Param(KeySpace.unsecurified(isHardened: false), 9, "9", "9"),
			Param(KeySpace.unsecurified(isHardened: true), 0x80000009, "9'", "9H"),
			Param(KeySpace.securified, 0xc0000009, "9^", "9S")
		]
	)
	func global_roundtrip(params: Param) throws {
		let keySpace = params.keySpace
		let global = params.global
		let description = params.description
		let debugDescription = params.debugDescription
		let sut = Sut(indexInGlobalKeySpace: global)
		#expect(sut.indexInLocalKeySpace() == 9)
		#expect(sut.keySpace == keySpace)
		#expect(sut.indexInGlobalKeySpace() == global)
		#expect(sut.description == description)
		#expect(sut.debugDescription == debugDescription)
	}



}

@Suite("Unhardened")
struct UnhardenedTests {
	typealias Sut = Unhardened
	
	@Test("From U31")
	func fromU31() throws {
		let sut = try Sut(u31: U31(value: 5))
		try #expect(Sut(localKeySpace: 5) == sut)
	}
	
	@Test
	func local() throws {
		for local in UInt32(0)...3 {
			let sut = try Sut(localKeySpace: local)
			let indexInLocal = sut.indexInLocalKeySpace()
			#expect(local == indexInLocal)
			#expect(try Sut(localKeySpace: indexInLocal) == sut)
		}
	}
	
	@Test
	func global() throws {
		for global in UInt32(0)...3 {
			let sut = try Sut(globalKeySpace: global)
			let indexInGlobal = sut.indexInGlobalKeySpace()
			#expect(global == indexInGlobal)
			#expect(try Sut(globalKeySpace: indexInGlobal) == sut)
		}
	}
}

@Suite("SecurifiedU30")
struct SecurifiedU30Tests {
	typealias Sut = SecurifiedU30
	
	@Test("From U30")
	func fromU30() throws {
		let sut = try Sut(u30: U30(value: 5))
		try #expect(Sut(localKeySpace: 5) == sut)
	}
	
	@Test
	func local() throws {
		for local in UInt32(0)...3 {
			let sut = try Sut(localKeySpace: local)
			let indexInLocal = sut.indexInLocalKeySpace()
			#expect(local == indexInLocal)
			#expect(try Sut(localKeySpace: indexInLocal) == sut)
		}
	}
	
	@Test
	func global() throws {
		for global in UInt32(0xc0000000)...0xc0000003 {
			let sut = try Sut(globalKeySpace: global)
			let indexInGlobal = sut.indexInGlobalKeySpace()
			#expect(global == indexInGlobal)
			#expect(try Sut(globalKeySpace: indexInGlobal) == sut)
		}
	}
}
