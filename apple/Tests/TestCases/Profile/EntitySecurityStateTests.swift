//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-27.
//

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class EntitySecurityStateTests: Test<EntitySecurityState> {}
	
#if DEBUG
extension EntitySecurityState {
	public var unsecured: UnsecuredEntityControl {
		get {
			switch self {
			case let .unsecured(value):
				return value
			}
		}
		set {
			self = .unsecured(value: newValue)
		}
	}
}
#endif // DEBUG
