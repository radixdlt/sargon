import Foundation
import SargonUniFFI

extension WalletToDappInteractionResponse {
	public init(jsonData: some DataProtocol) throws {
		self = try newWalletToDappInteractionResponseFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		walletToDappInteractionResponseToJsonBytes(walletToDappInteractionResponse: self)
	}
}
