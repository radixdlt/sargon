import Foundation
import SargonUniFFI

extension Gateways {
	public init(current: Gateway) {
		self = newGateways(current: current)
	}

	public var all: [Gateway] {
		gatewaysGetAllElements(gateways: self)
	}

	public static let `default`: Self = newGatewaysDefault()

	/// Swaps current and other gateways:
	///
	/// * Adds (old)`current` to `other` (throws error if it was already present)
	/// * Removes `newCurrent` from `other` (if present)
	/// * Sets `current = newCurrent`
	public mutating func changeCurrent(
		to newCurrent: Gateway
	) throws {
		self = try newGatewaysChangingCurrent(
			to: newCurrent,
			gateways: self
		)
	}
}
