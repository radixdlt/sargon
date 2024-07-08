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

final class AppPreferencesTests: Test<AppPreferences> {
	func test_default_guarantee_is_99() {
		XCTAssertEqual(SUT.default.transaction.defaultDepositGuarantee, 0.99)
	}
	
	func test_has_gateway() {
		XCTAssertTrue(SUT.default.hasGateway(with: .init(string: "https://mainnet.radixdlt.com/")!))
		XCTAssertTrue(SUT.default.hasGateway(with: .init(string: "https://mainnet.radixdlt.com")!))
		XCTAssertFalse(SUT.default.hasGateway(with: .init(string: "https://radixdlt.com")!))
	}
}
