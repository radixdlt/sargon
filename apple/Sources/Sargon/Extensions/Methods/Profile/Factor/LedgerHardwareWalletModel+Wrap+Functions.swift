import Foundation
import SargonUniFFI

extension LedgerHardwareWalletModel {
	public init(string: String) throws {
		self = try newLedgerHwWalletModelFromString(string: string)
	}

	public func toString() -> String {
		ledgerHwWalletModelToString(model: self)
	}
}
