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
    
    public var summary: ManifestSummary? {
        transactionManifestSummary(manifest: self)
    }
    
	/// Creates the `ExecutionSummary` based on the `engineToolkitReceipt` data. 
    /// 
    /// Such value should be obtained from the Gateway `/transaction/preview` endpoint, under the `radix_engine_toolkit_receipt` field.
    /// Its content will be parsed into a `String` representation and used as parameter here.
    public func executionSummary(engineToolkitReceipt: String) throws -> ExecutionSummary {
        try transactionManifestExecutionSummary(
            manifest: self,
            engineToolkitReceipt: engineToolkitReceipt
        )
    }
}
