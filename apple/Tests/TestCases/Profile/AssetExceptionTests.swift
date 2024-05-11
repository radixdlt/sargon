//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AssetExceptionTests: Test<AssetException> {
	func test_id_is_address() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.address)
		}
	}
}
