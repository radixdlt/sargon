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
		var sut = SUT(resourceFlags: [:])
		XCTAssertTrue(sut.hiddenResources.isEmpty)
		
		sut.hideResource(resource: .sampleOther)
		sut.hideResource(resource: .sample)
		XCTAssertEqual([.sampleOther, .sample], sut.hiddenResources)
	}
	
	func test_hide_unhide() {
		var sut = SUT(resourceFlags: [:])
		let resource: ResourceAddress = .sample
		XCTAssertFalse(sut.hasResourceHidden(resource: resource))
		
		sut.hideResource(resource: resource)
		XCTAssertTrue(sut.hasResourceHidden(resource: .sample))
		
		sut.unhideResource(resource: resource)
		XCTAssertFalse(sut.hasResourceHidden(resource: .sample))
	}
}
