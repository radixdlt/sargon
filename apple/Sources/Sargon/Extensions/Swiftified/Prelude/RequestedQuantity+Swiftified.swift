//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

extension RequestedQuantity: SargonModel {}
extension RequestedQuantity: SargonObjectCodable {}

extension RequestedQuantity {

	public static func exactly(_ quantity: Int) -> Self {
		.init(quantifier: .exactly, quantity: UInt16(quantity))
	}

	public static func atLeast(_ quantity: Int) -> Self {
		.init(quantifier: .atLeast, quantity: UInt16(quantity))
	}
}
