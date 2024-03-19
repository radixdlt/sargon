extension NonFungibleResourceAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newNonFungibleResourceAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		nonFungibleResourceAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		nonFungibleResourceAddressNetworkId(address: self)
	}
}
