extension TransactionManifest: CustomStringConvertible {
	public var description: String {
		transactionManifestToString(manifest: self)
	}
}
