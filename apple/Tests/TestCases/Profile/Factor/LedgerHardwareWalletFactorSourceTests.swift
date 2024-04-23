//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class LedgerHardwareWalletFactorSourceTests: FactorSourceTest<LedgerHardwareWalletFactorSource> {
	func test_id_of_ledger() {
		XCTAssertEqual(SUT.sample.id.description, FactorSourceID.hash(value: SUT.sample.id).description)
	}
	
	func test_factor_source_id_is_id() {
		XCTAssertEqual(SUT.sample.id.asGeneral, SUT.sample.factorSourceID)
	}
	
	func test_kind() {
		XCTAssertEqual(SUT.sample.factorSourceKind, .ledgerHqHardwareWallet)
	}
	
	func test_as_factor_source_to_string() {
		XCTAssertEqual(SUT.sample.asGeneral.id.description, SUT.sample.id.description)
	}
	
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSource.ledger(value: SUT.sample))
	}
    
    func test_source_that_supports_babylon() {
        let sut = SUT.sample
        XCTAssertTrue(sut.supportsBabylon)
        XCTAssertFalse(sut.supportsOlympia)
    }
    
    func test_source_that_supports_olympia() {
        let sut = SUT.sampleOther
        XCTAssertTrue(sut.supportsOlympia)
        XCTAssertFalse(sut.supportsBabylon)
    }
    
  
}
