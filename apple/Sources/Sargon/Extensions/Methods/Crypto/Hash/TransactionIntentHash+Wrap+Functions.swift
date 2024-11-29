import SargonUniFFI

extension TransactionIntentHash {
	public init(_ string: String) throws {
		self = try newTransactionIntentHashFromString(string: string)
	}

	public func formatted(_ format: AddressFormat = .default) -> String {
		transactionIntentHashFormatted(address: self, format: format)
	}
}
