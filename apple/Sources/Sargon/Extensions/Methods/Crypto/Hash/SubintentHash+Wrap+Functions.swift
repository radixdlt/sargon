extension SubintentHash {
	public func formatted(_ format: AddressFormat = .default) -> String {
		subintentHashFormatted(address: self, format: format)
	}
}
