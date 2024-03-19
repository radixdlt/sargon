extension ResourceAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newResourceAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		resourceAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		resourceAddressNetworkId(address: self)
	}

	/// If this is an address of a **fungible** resource or not.
	public var isFungible: Bool {
		resourceAddressIsFungible(address: self)
	}

	/// If this is an address of a **non-fungible** resource or not.
	public var isNonFungible: Bool {
		resourceAddressIsNonFungible(address: self)
	}
}
