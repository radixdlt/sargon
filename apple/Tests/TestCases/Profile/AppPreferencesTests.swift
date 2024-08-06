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
	
	func test_has_gateway_valid() throws {
		XCTAssertTrue(try SUT.default.hasGateway(with: .init(string: "https://mainnet.radixdlt.com/")!))
		XCTAssertTrue(try SUT.default.hasGateway(with: .init(string: "https://mainnet.radixdlt.com")!))
		XCTAssertFalse(try SUT.default.hasGateway(with: .init(string: "https://radixdlt.com")!))
		
	}
	
	func test_has_gateway_invalid() {
		let url = URL(string: "Rust considers this invalid")
		XCTAssertNotNil(url)
		XCTAssertThrowsError(try SUT.default.hasGateway(with: url!)) { error in
			XCTAssertEqual(error.localizedDescription, "Failed to convert arg \'url\': relative URL without a base")
		}
	}
}
