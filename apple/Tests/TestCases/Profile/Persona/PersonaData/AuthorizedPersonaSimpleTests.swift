//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-21.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AuthorizedPersonaSimpleTests: Test<AuthorizedPersonaSimple> {
	func test_network_ids_mainnet() {
		XCTAssertTrue(SUT.sampleValuesMainnet.allSatisfy({ $0.networkID == .mainnet }))
	}
	
	func test_network_ids_stokenet() {
		XCTAssertTrue(SUT.sampleValuesStokenet.allSatisfy({ $0.networkID == .stokenet }))
	}
}
