extension TransactionManifest: CustomStringConvertible {
	public var description: String { instructionsString }
}

extension TransactionManifest {

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
