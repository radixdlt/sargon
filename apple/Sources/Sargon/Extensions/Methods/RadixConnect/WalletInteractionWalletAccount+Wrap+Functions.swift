import Foundation
import SargonUniFFI

extension WalletInteractionWalletAccount {
	public init(jsonData: some DataProtocol) throws {
		self = try newWalletInteractionWalletAccountFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		walletInteractionWalletAccountToJsonBytes(walletInteractionWalletAccount: self)
	}
}
