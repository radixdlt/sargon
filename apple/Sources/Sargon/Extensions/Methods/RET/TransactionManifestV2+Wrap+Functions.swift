import Foundation
import SargonUniFFI

extension TransactionManifestV2: CustomStringConvertible {
	public var description: String { manifestString }
}

extension TransactionManifestV2 {

	public var manifestString: String {
		transactionManifestStringV2(manifest: self)
	}

	public var networkID: NetworkID {
		transactionManifestNetworkIdV2(manifest: self)
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
		transactionManifestSummaryV2(manifest: self)
	}
}
