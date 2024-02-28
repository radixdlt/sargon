extension TransactionManifest: @unchecked Sendable {}

extension TransactionManifest: CustomStringConvertible {
	public var description: String {
		transactionManifestToString(manifest: self)
	}
}

#if DEBUG
	extension TransactionManifest {
		public static let sample: Self = newTransactionManifestSample()
		public static let sampleOther: Self = newTransactionManifestSampleOther()
	}
#endif
