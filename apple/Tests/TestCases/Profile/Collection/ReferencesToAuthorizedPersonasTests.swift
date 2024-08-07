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

final class ReferencesToAuthorizedPersonasTests: CollectionTest<AuthorizedPersonaSimple> {
	
	override class func sample() -> SUT {
		SUT.sample
	}
	
	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}
}
