import Foundation
import SargonUniFFI

extension TransactionManifest: CustomStringConvertible {
	public var description: String { instructionsString }
}

extension TransactionManifest {
	
	public init(instructionsString: String, networkID: NetworkID, blobs: Blobs = []) throws {
		self = try newTransactionManifestFromInstructionsStringAndBlobs(
			instructionsString: instructionsString,
			networkId: networkID,
			blobs: blobs
		)
	}

	public var manifestString: String {
		transactionManifestString(manifest: self)
	}

	public var instructionsString: String {
		transactionManifestInstructionsString(manifest: self)
	}

	public var networkID: NetworkID {
		transactionManifestNetworkId(manifest: self)
	}

	public var blobs: Blobs {
		transactionManifestBlobs(manifest: self)
	}
    
    public var involvedPoolAddresses: [PoolAddress] {
        transactionManifestInvolvedPoolAddresses(manifest: self)
    }
    
    public var involvedResourceAddresses: [ResourceAddress] {
        transactionManifestInvolvedResourceAddresses(manifest: self)
    }
    
    public var summary: ManifestSummary {
        get throws {
            try transactionManifestSummary(manifest: self)
        }
    }
}
