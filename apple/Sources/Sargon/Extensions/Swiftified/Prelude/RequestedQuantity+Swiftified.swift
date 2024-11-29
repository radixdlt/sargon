import Foundation
import SargonUniFFI

// MARK: - RequestedQuantity + SargonModel
extension RequestedQuantity: SargonModel {}

// MARK: - RequestedQuantity + SargonObjectCodable
extension RequestedQuantity: SargonObjectCodable {}

extension RequestedQuantity {
	public static func exactly(_ quantity: Int) -> Self {
		.init(quantifier: .exactly, quantity: UInt16(quantity))
	}

	public static func atLeast(_ quantity: Int) -> Self {
		.init(quantifier: .atLeast, quantity: UInt16(quantity))
	}
}
