//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-18.
//

import Foundation
import SargonUniFFI

extension SavedGateways {
	
	public init(current: Gateway) {
		self = newSavedGateways(current: current)
	}
	
	public var all: [Gateway] {
		savedGatewaysGetAllElements(gateways: self)
	}
	
	public static let `default`: Self = newSavedGatewaysDefault()
	
	/// Swaps current and other gateways:
	///
	/// * Adds (old)`current` to `other` (throws error if it was already present)
	/// * Removes `newCurrent` from `other` (if present)
	/// * Sets `current = newCurrent`
	public mutating func changeCurrent(
		to newCurrent: Gateway
	) throws {
		self = try newSavedGatewaysChangingCurrent(
			to: newCurrent,
			gateways: self
		)
	}

}
