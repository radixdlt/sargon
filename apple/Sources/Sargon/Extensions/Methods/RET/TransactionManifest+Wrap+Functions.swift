extension TransactionManifest: CustomStringConvertible {
	public var description: String { instructionsString }
}

extension TransactionManifest {
	
	public init(instructionsString: String, networkID: NetworkID, blobs: Blobs) throws {
		self = try newTransactionManifestFromInstructionsStringAndBlobs(
			instructionsString: instructionsString,
			networkId: networkID,
			blobs: blobs
		)
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
}
