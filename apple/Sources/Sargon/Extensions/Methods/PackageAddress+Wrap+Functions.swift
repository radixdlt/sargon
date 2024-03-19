extension PackageAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newPackageAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		packageAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		packageAddressNetworkId(address: self)
	}
}
