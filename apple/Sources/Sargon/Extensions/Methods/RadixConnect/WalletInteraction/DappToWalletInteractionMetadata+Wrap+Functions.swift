import Foundation
import SargonUniFFI

extension DappToWalletInteractionMetadata {
	public init(jsonData: some DataProtocol) throws {
		self = try newDappToWalletInteractionMetadataFromJsonBytes(
			jsonBytes: Data(jsonData)
		)
	}

	public func jsonData() -> Data {
		dappToWalletInteractionMetadataToJsonBytes(
			dappToWalletInteractionMetadata: self
		)
	}
}
