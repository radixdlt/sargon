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

final class ResourcePreferencesTests: Test<ResourcePreferences> {
	func test_hidden_resources() {
		var sut = SUT(fungible: [:], nonFungible: [:], poolUnit: [:])
		XCTAssertTrue(sut.hiddenResources.fungible.isEmpty)
		XCTAssertTrue(sut.hiddenResources.nonFungible.isEmpty)
		XCTAssertTrue(sut.hiddenResources.poolUnit.isEmpty)
		
		// Hide resources
		sut.hideResource(kind: .fungible(.sample))
		sut.hideResource(kind: .fungible(.sampleOther))
		sut.hideResource(kind: .nonFungible(.sample))
		sut.hideResource(kind: .poolUnit(.sample))
		
		XCTAssertEqual([.sampleOther, .sample], sut.hiddenResources.fungible)
		XCTAssertEqual([.sample], sut.hiddenResources.nonFungible)
		XCTAssertEqual([.sample], sut.hiddenResources.poolUnit)
		
		// Unhide resources
		sut.unhideResource(kind: .fungible(.sampleOther))
		sut.unhideResource(kind: .nonFungible(.sample))
		XCTAssertEqual([.sample], sut.hiddenResources.fungible)
		XCTAssertTrue(sut.hiddenResources.nonFungible.isEmpty)
		XCTAssertEqual([.sample], sut.hiddenResources.poolUnit)
	}
}
