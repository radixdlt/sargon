import Foundation
import SargonUniFFI

extension DappToWalletInteractionSubintentExpiration {
	public func getStatus() -> DappToWalletInteractionSubintentExpirationStatus {
		getSubintentExpirationStatus(expiration: self)
	}
}
