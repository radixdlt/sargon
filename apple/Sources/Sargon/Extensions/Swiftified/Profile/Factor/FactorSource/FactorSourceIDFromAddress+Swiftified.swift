//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromAddress = FactorSourceIdFromAddress

extension FactorSourceIDFromAddress: SargonModel {}
extension FactorSourceIDFromAddress: SargonObjectCodable {}

extension FactorSourceIDFromAddress {
	public var asGeneral: FactorSourceID {
		.address(value: self)
	}
}


extension FactorSourceIDFromAddress: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
