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
		// Valid URLs
		do {
			var result = try SUT.default.hasGateway(with: .init(string: "https://mainnet.radixdlt.com/")!)
			XCTAssertTrue(result)
			result = try SUT.default.hasGateway(with: .init(string: "https://mainnet.radixdlt.com")!)
			XCTAssertTrue(result)
			result = try SUT.default.hasGateway(with: .init(string: "https://radixdlt.com")!)
			XCTAssertFalse(result)
		} catch {
			XCTFail("Unexpected failure")
		}
		
		// Invalid URL
		do {
			let url = URL(string: "Rust considers this invalid")
			XCTAssertNotNil(url)
			let _ = try SUT.default.hasGateway(with: url!)
			XCTFail("Should have thrown")
		} catch let error {
			XCTAssertEqual(error.localizedDescription, "Failed to convert arg \'url\': relative URL without a base")
		}
	}
}
