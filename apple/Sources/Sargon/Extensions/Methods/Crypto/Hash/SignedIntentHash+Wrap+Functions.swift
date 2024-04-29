import SargonUniFFI

extension SignedIntentHash {
	public init(_ string: String) throws {
		self = try newSignedIntentHashFromString(string: string)
	}

	public func formatted(_ format: AddressFormat = .default) -> String {
		signedIntentHashFormatted(address: self, format: format)
	}
}
