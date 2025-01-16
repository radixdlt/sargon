import Foundation
import SargonUniFFI

extension TimePeriodUnit {
	public var values: [Int] {
		Array(1 ... Int(constantMaxRecoveryConfirmationFallbackPeriodUnits()))
	}
}
