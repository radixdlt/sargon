import Foundation
import SargonUniFFI

extension TimePeriod {
	public init(days: Int) {
		self = newTimePeriodWithDays(value: UInt16(days))
	}

	public func days() -> Int {
		Int(timePeriodToDays(timePeriod: self))
	}
}
