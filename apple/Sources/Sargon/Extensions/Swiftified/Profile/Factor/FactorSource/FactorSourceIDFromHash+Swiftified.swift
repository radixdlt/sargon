//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromHash = FactorSourceIdFromHash

extension FactorSourceIDFromHash {
	public var asGeneral: FactorSourceID {
		.hash(value: self)
	}
}
