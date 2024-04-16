//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

extension FactorSourceIDFromAddress {
	public func toString() -> String {
		factorSourceIdFromAddressToString(factorSourceId: self)
	}
}
