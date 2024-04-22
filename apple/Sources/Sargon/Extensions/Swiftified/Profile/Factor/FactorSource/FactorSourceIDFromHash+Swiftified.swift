//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromHash = FactorSourceIdFromHash

extension FactorSourceIDFromHash: SargonModel {}
extension FactorSourceIDFromHash: SargonObjectCodable {}

extension FactorSourceIDFromHash {
	public var asGeneral: FactorSourceID {
		.hash(value: self)
	}
}

extension FactorSourceIDFromHash: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
