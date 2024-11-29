import Foundation

// MARK: - LedgerHardwareWalletModel + SargonModel
extension LedgerHardwareWalletModel: SargonModel {}

// MARK: - LedgerHardwareWalletModel + CustomStringConvertible
extension LedgerHardwareWalletModel: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

extension LedgerHardwareWalletModel {
	public var rawValue: String {
		toString()
	}
}
