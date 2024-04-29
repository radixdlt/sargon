import SargonUniFFI

extension IntentHash {
	public init(_ string: String) throws {
		self = try newIntentHashFromString(string: string)
	}

	public func formatted(_ format: AddressFormat = .default) -> String {
		intentHashFormatted(address: self, format: format)
	}
}
