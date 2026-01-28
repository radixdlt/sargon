import Foundation
import SargonUniFFI

// MARK: - TransactionManifestV2 + CustomStringConvertible
extension TransactionManifestV2: CustomStringConvertible {
	public var description: String {
		manifestString
	}
}

extension TransactionManifestV2 {
	public var manifestString: String {
		transactionManifestStringV2(manifest: self)
	}

	public var blobs: Blobs {
		transactionManifestBlobsV2(manifest: self)
	}

	public var involvedPoolAddresses: [PoolAddress] {
		transactionManifestInvolvedPoolAddressesV2(manifest: self)
	}

	public var involvedResourceAddresses: [ResourceAddress] {
		transactionManifestInvolvedResourceAddressesV2(manifest: self)
	}

	public var summary: ManifestSummary? {
		try? transactionManifestSummaryV2(manifest: self)
	}
}
