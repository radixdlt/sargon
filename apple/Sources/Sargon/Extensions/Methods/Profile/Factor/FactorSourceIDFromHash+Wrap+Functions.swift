//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

extension FactorSourceIDFromHash {
	public func toString() -> String {
		factorSourceIdFromHashToString(factorSourceId: self)
	}
}
