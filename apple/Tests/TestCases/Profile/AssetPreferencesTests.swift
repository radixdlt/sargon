//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 13/8/24.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AssetPreferencesTests: TestCase {
	func test_hidden_resources() {
		var sut = AssetPreferences()
		XCTAssertTrue(sut.hiddenAssets.isEmpty)
		
		// Hide assets
		sut.hideAsset(asset: .fungible(.sample))
		sut.hideAsset(asset: .nonFungible(.sample))
		sut.hideAsset(asset: .fungible(.sampleOther))
		
		XCTAssertEqual(sut.hiddenAssets, [.fungible(.sample), .nonFungible(.sample), .fungible(.sampleOther)])
		
		// Unhide assets
		sut.unhideAsset(asset: .fungible(.sampleOther))
		sut.unhideAsset(asset: .nonFungible(.sample))
		XCTAssertEqual(sut.hiddenAssets, [.fungible(.sample)])
	}
}
