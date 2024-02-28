extension TransactionManifest: @unchecked Sendable {}

extension TransactionManifest: CustomStringConvertible {
	public var description: String {
		transactionManifestToString(manifest: self)
	}
}

#if DEBUG
extension TransactionManifest {
	public static let placeholder: Self = newTransactionManifestPlaceholder()
	public static let placeholderOther: Self = newTransactionManifestPlaceholderOther()
}
#endif
